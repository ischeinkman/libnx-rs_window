
use libnx_rs::libnx;
use libnx_rs::libnx::{HidControllerID, hidScanInput};

use window::{BuildFromWindowSettings, Window, WindowSettings, Size, Position};
use input::Input;
use std::vec::Vec;
use std::time::{Duration, Instant};

use controller;
use controller::LibnxButtonId;

pub struct NxFullWindow {
    size : Size, 
    framebuffer : *mut u8,
    should_close : bool,
    event_backlog : Vec<Input>,
}

impl NxFullWindow {

    pub fn new() -> NxFullWindow {
        unsafe { libnx::gfxInitDefault() };
        let mut width : u32 = 0;
        let mut height : u32 = 0;

        let framebuffer = unsafe {
            (libnx::gfxGetFramebuffer(&mut width as *mut u32, &mut height as *mut u32)) as *mut u32 as *mut u8
        };

        let sz = Size {width, height};

        NxFullWindow {
            size : sz,
            framebuffer : framebuffer,
            should_close : false, 
            event_backlog : Vec::new(),
        }
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

    pub unsafe fn get_framebuffer(&mut self) -> *mut u8 {
        *(&mut self.framebuffer as *mut *mut u8)
    }
}

impl Drop for NxFullWindow {
    fn drop(&mut self) {
        unsafe { libnx::gfxExit() };
    }
}

impl BuildFromWindowSettings for NxFullWindow {
    fn build_from_window_settings(settings : &WindowSettings) -> Result<Self, String> {
        //TODO: Settings?
        Ok(NxFullWindow::new())
    }
}

impl Window for NxFullWindow {
    fn set_should_close(&mut self, value : bool) {
        self.should_close = value;
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn size(&self) -> Size {
        self.size
    }

    fn swap_buffers(&mut self) {
        unsafe {
            libnx::gfxFlushBuffers();
            libnx::gfxSwapBuffers();
            libnx::gfxWaitForVsync();
        }
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
        self.size
    }
}