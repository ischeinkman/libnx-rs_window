
// TODO: make this dependent on whether libnx-rs is loaded
// from the sysroot
extern crate libnx_rs;
extern crate window; 
extern crate input; 

extern crate egl;

mod controller;

pub use controller::LibnxButtonId;

mod simple;
pub use simple::NxFullWindow;

mod gl_window;
pub use gl_window::NxGlWindow;