// warnings
#![allow(dangerous_implicit_autorefs)]
#![allow(unsafe_op_in_unsafe_fn)]

// imports
extern crate piston_window;
use std::ffi::{c_char, CStr};
use piston_window::*;

/// Window
static mut WINDOW: *mut PistonWindow = std::ptr::null_mut();

/// Creates string from *const char
pub unsafe fn to_string(string: *const c_char) -> String {
    CStr::from_ptr(string).to_string_lossy().into_owned()
}

/// Initializes window
#[unsafe(no_mangle)]
pub unsafe fn init(raw_title: *const c_char, width: u32, height: u32) {
    // title
    let title = to_string(raw_title);
    drop(Box::from_raw(raw_title as *mut c_char));
    // log
    println!("[butterfly] init");
    // creating window
    WINDOW = Box::into_raw(Box::new(
        WindowSettings::new(title, [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap(),
    ));
}

/// Ticks window
#[unsafe(no_mangle)]
pub unsafe fn tick() {
    // log
    println!("[butterfly] tick");
    // draw
    if let Some(e) = (*WINDOW).next() {
        (*WINDOW).draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });
    }
}