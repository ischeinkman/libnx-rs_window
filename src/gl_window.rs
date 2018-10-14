
use egl;
use std::ptr;

use libnx_rs::libnx;
use libnx_rs::libnx::{HidControllerID, hidScanInput};

use window::{BuildFromWindowSettings, Window, WindowSettings, Size, Position, OpenGLWindow, AdvancedWindow, ProcAddress};
use input::Input;
use std::vec::Vec;
use std::time::{Duration, Instant};

use controller;
use controller::LibnxButtonId;


// Shamelessly stealing from the switchbrew example.
unsafe fn init_egl() -> Result<(egl::EGLDisplay, egl::EGLContext, egl::EGLSurface), String> {
    let display = egl::get_display(egl::EGL_DEFAULT_DISPLAY)
        .ok_or(format!("Could not connect to display! Error {}.", egl::get_error()))?;

    let mut _a = 0;
    let mut _b = 0;
    egl::initialize(display, &mut _a, &mut _b);
    
    if !egl::bind_api(egl::EGL_OPENGL_API) {
        egl::terminate(display);
        return Err(format!("Could not set API! Error: {}.", egl::get_error()));
    }

    let framebuffer_attribute_list : [egl::EGLint; 7] = [
        egl::EGL_RED_SIZE, 1,
        egl::EGL_GREEN_SIZE, 1,
        egl::EGL_BLUE_SIZE, 1, 
        egl::EGL_NONE
    ];
    let config = match egl::choose_config(display, &framebuffer_attribute_list, 1) {
        Some(cfg) => cfg,
        None => {
            egl::terminate(display);
            return Err(format!("No config found! Error: {}.", egl::get_error()));
        }
    };
    let surface = match egl::create_window_surface(display, config, "".as_ptr() as *mut _, &[]) {
        Some(sf) => sf,
        None => {
            egl::terminate(display);
            return Err(format!("Surface creation failed! Error: {}.", egl::get_error()));
        }
    };
    let context_attrib_list = [

    ];
    let context = match egl::create_context(display, config, egl::EGL_NO_CONTEXT, &context_attrib_list) {
        Some(ctx) => ctx,
        None => {
            egl::destroy_surface(display, surface);
            egl::terminate(display);
            return Err(format!("Context creation failed! Error: {}.", egl::get_error()));
        }
    };

    egl::make_current(display, surface, surface, context);
    Ok((display, context, surface))
}

unsafe fn deinit_egl(display : Option<egl::EGLDisplay>, context : Option<egl::EGLContext>, surface : Option<egl::EGLSurface>) {
    let disp = match display {
        Some(d) => d,
        None => {
            return;
        }
    };

    egl::make_current(disp, egl::EGL_NO_SURFACE, egl::EGL_NO_SURFACE, egl::EGL_NO_CONTEXT);
    if let Some(ctx) = context {
        egl::destroy_context(disp, ctx);
    }
    if let Some(srf) = surface {
        egl::destroy_surface(disp, srf);
    }
    egl::terminate(disp);
} 

pub struct NxGlWindow {
    display : egl::EGLDisplay,
    context : egl::EGLContext,
    surface : egl::EGLSurface,
    should_close : bool,
    event_backlog : Vec<Input>,
}

impl NxGlWindow {

    pub fn new() -> Result<NxGlWindow, String> {

        let (display, context, surface) = unsafe { init_egl()? };

        Ok(NxGlWindow {
            display,
            context,
            surface,
            should_close : false, 
            event_backlog : Vec::new(),
        })
    }

    fn check_inputs(&mut self) {
        unsafe { 
            hidScanInput();
            let kDown = libnx::hidKeysDown(HidControllerID::CONTROLLER_P1_AUTO) as u32;
            let parse_events_d = controller::parse_key_events(1, controller::LibnxKeyState::Down, kDown);
            self.event_backlog.extend(parse_events_d);

            let kUp = libnx::hidKeysUp(HidControllerID::CONTROLLER_P1_AUTO) as u32;
            let parse_events_u = controller::parse_key_events(1, controller::LibnxKeyState::Up, kUp);
            self.event_backlog.extend(parse_events_u);

            let kHeld = libnx::hidKeysHeld(HidControllerID::CONTROLLER_P1_AUTO) as u32;
            let parse_events_h = controller::parse_key_events(1, controller::LibnxKeyState::Held, kHeld);
            self.event_backlog.extend(parse_events_h);
        }
    }

}

impl Drop for NxGlWindow {
    fn drop(&mut self) {
        unsafe { deinit_egl(Some(self.display), Some(self.context), Some(self.surface)) };
    }
}

impl BuildFromWindowSettings for NxGlWindow {
    fn build_from_window_settings(settings : &WindowSettings) -> Result<Self, String> {
        //TODO: Settings?
        NxGlWindow::new()
    }
}

impl Window for NxGlWindow {
    fn set_should_close(&mut self, value : bool) {
        self.should_close = value;
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn size(&self) -> Size {
        //TODO: Howto get size?
        Size {
            width : 1280,
            height : 720
        }
    }

    fn swap_buffers(&mut self) {
        egl::swap_buffers(self.display, self.surface);
    }

    fn wait_event(&mut self) -> Input {
        loop {
            let evt = self.poll_event();
            if evt.is_some() {
                return evt.unwrap();
            }
        }
    }

    fn poll_event(&mut self) -> Option<Input> {
        self.check_inputs();
        self.event_backlog.pop()
    }

    fn wait_event_timeout(&mut self, timeout : Duration) -> Option<Input> {
        let t_start = Instant::now();
        let mut t_cur = Instant::now();
        while t_cur.duration_since(t_start) <= timeout {
            let evt = self.poll_event();
            if evt.is_some() {
                return evt;
            }
            t_cur = Instant::now();
        }
        None
    }

    fn draw_size(&self) -> Size {
        self.size()
    }
}

impl OpenGLWindow for NxGlWindow {
    fn get_proc_address(&mut self, proc_name : &str) -> ProcAddress {
        egl::get_proc_address(proc_name) as ProcAddress
    }

    fn is_current(&self) -> bool {
        true //Currently libnx-rs only allows for 1 window at a time
    }

    fn make_current(&mut self) {
        //Currently unnecessary
    }
}