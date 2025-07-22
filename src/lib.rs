// warnings
#![allow(dangerous_implicit_autorefs)]
#![allow(unsafe_op_in_unsafe_fn)]

// imports
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture, Sdl2ImageContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

/// Sdl
pub struct SDL {
    pub sdl: *mut Sdl,
    pub video_ctx: *mut VideoSubsystem,
    pub canvas: *mut sdl2::render::Canvas<sdl2::video::Window>,
    pub texture_creator: *mut sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub textures: *mut Vec<Texture<'static>>,
    pub image_ctx: *mut Sdl2ImageContext,
    pub event_pump: *mut EventPump,
}
unsafe impl Send for SDL {}
unsafe impl Sync for SDL {}

/// Statics
pub static mut SDL2: SDL = SDL {
    sdl: std::ptr::null_mut(),
    video_ctx: std::ptr::null_mut(),
    canvas: std::ptr::null_mut(),
    texture_creator: std::ptr::null_mut(),
    textures: std::ptr::null_mut(),
    image_ctx: std::ptr::null_mut(),
    event_pump: std::ptr::null_mut(),
};

/// Init
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {
    println!("[butterfly] init.");
    println!("[butterfly] init sdl2.");
    match sdl2::init() {
        Ok(sdl) => {
            SDL2.sdl = Box::into_raw(Box::new(sdl));
            println!("[butterfly] sdl2::init() : ok");
        }
        Err(error) => {
            panic!("[butterfly] sdl2::init() : error: {:?}", error);
        }
    }
    println!("[butterfly] init sdl2 video subsystem.");
    match (*SDL2.sdl).video() {
        Ok(video_ctx) => {
            SDL2.video_ctx = Box::into_raw(Box::new(video_ctx));
            println!("[butterfly] sdl2::video() : ok");
        }
        Err(error) => {
            panic!("[butterfly] sdl2::video() : error: {:?}", error);
        }
    }
    println!("[butterfly] init sdl2 window (test, 1080, 720).");
    match (*SDL2.video_ctx).window("test", 1080, 720)
        .position_centered().build() {
        Ok(window) => {
            match window.into_canvas().accelerated().build() {
                Ok(canvas) => {
                    println!("[butterfly] init sdl2 canvas.");
                    SDL2.canvas = Box::into_raw(Box::new(canvas));
                }
                Err(error) => {
                    panic!("[butterfly] sdl2::window::canvas() : error: {:?}", error);
                }
            }
        }
        Err(error) => {
            panic!("[butterfly] init sdl2.window() : error: {:?}", error);
        }
    }
    println!("[butterfly] init sdl2 texture creator.");
    SDL2.texture_creator = Box::into_raw(Box::new((*SDL2.canvas).texture_creator()));
    let textures: Vec<Texture> = Vec::new();
    SDL2.textures = Box::into_raw(Box::new(textures));
    println!("[butterfly] init sdl2 image ctx.");
    match sdl2::image::init(InitFlag::PNG | InitFlag::JPG) {
        Ok(ok) => {
            SDL2.image_ctx = Box::into_raw(Box::new(ok))
        }
        Err(err) => {
            panic!("[butterfly] sdl2::image::init() : error: {:?}", err);
        }
    }
    println!("[butterfly] init sdl2 event pump.");
    match (*SDL2.sdl).event_pump() {
        Ok(ok) => {
            SDL2.event_pump = Box::into_raw(Box::new(ok))
        }
        Err(err) => {
            panic!("[butterfly] sdl2::event_pump() : error: {:?}", err);
        }
    }
    println!("[butterfly] all done.");

}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_screen() {
    (*SDL2.canvas).set_draw_color(Color::RGB(0, 0, 0));
    (*SDL2.canvas).clear();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn redraw_screen() {
    (*SDL2.canvas).present();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_texture(raw_path: *mut String) -> usize {
    let path = (*raw_path).clone();
    match (*SDL2.texture_creator).load_texture(path) {
        Ok(texture) => {
            (*SDL2.textures).push(texture);
            (*SDL2.textures).len() - 1
        }
        Err(error) => {
            panic!("[butterfly] failed to load texture : {}", error);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn draw_texture(id: usize, x: i32, y: i32, width: u32, height: u32) {
    let result = (*SDL2.canvas).copy(
        &(*SDL2.textures)[id],
        None,
        Some(Rect::new(x, y, width, height)),
    );
    match result {
        Err(e) => panic!("[butterfly] failed to copy texture to canvas: {}", e),
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_resources() {
    macro_rules! free_ptr {
        ($field:ident) => {
            if !SDL2.$field.is_null() {
                drop(Box::from_raw(SDL2.$field));
                SDL2.$field = std::ptr::null_mut();
            }
        };
    }

    free_ptr!(textures);
    free_ptr!(texture_creator);
    free_ptr!(canvas);
    free_ptr!(image_ctx);
    free_ptr!(video_ctx);
    free_ptr!(sdl);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn poll_event() -> *mut String {
    match (*SDL2.event_pump).poll_event() {
        None => {
            Box::into_raw(Box::new(
                String::from("nop")
            ))
        }
        Some(event) => {
            match event {
                Event::Quit { .. } => {
                    Box::into_raw(Box::new(String::from("app_quit")))
                }
                Event::AppTerminating { .. } => {
                    Box::into_raw(Box::new(String::from("app_terminating")))
                }
                Event::AppLowMemory { .. } => {
                    Box::into_raw(Box::new(String::from("app_low_memory")))
                }
                Event::AppWillEnterBackground { .. } => {
                    Box::into_raw(Box::new(String::from("app_will_enter_background")))
                }
                Event::AppDidEnterBackground { .. } => {
                    Box::into_raw(Box::new(String::from("app_did_enter_background")))
                }
                Event::AppWillEnterForeground { .. } => {
                    Box::into_raw(Box::new(String::from("app_will_enter_foreground")))
                }
                Event::AppDidEnterForeground { .. } => {
                    Box::into_raw(Box::new(String::from("app_did_enter_foreground")))
                }
                Event::Display { .. } => {
                    Box::into_raw(Box::new(String::from("app_display")))
                }
                Event::Window { .. } => {
                    Box::into_raw(Box::new(String::from("app_window")))
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        None => {
                            Box::into_raw(Box::new(String::from("undefined_key_down")))
                        }
                        Some(code) => {
                            Box::into_raw(Box::new(String::from(
                                format!("key_down {}", code.into_i32())
                            )))
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        None => {
                            Box::into_raw(Box::new(String::from("undefined_key_up")))
                        }
                        Some(code) => {
                            Box::into_raw(Box::new(String::from(
                                format!("key_up {}", code.into_i32())
                            )))
                        }
                    }
                }
                Event::TextEditing { text, .. } => {
                    Box::into_raw(Box::new(String::from(
                        format!("edit_text {}", text)
                    )))
                }
                Event::TextInput { text, .. } => {
                    Box::into_raw(Box::new(String::from(
                        format!("input_text {}", text)
                    )))
                }
                Event::MouseMotion { x, y, .. } => {
                    Box::into_raw(Box::new(String::from(
                        format!("mouse_motion {} {}", x, y)
                    )))
                }
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    Box::into_raw(Box::new(String::from(
                        format!("mouse_button_down {:?} {} {}", mouse_btn, x, y),
                    )))
                }
                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    Box::into_raw(Box::new(String::from(
                        format!("mouse_button_up {:?} {} {}", mouse_btn, x, y),
                    )))
                }
                Event::MouseWheel { x, y, mouse_x, mouse_y, .. } => {
                    Box::into_raw(Box::new(String::from(
                        format!("mouse_wheel {} {} {} {}", x, y, mouse_x, mouse_y),
                    )))
                }
                _ => {
                    Box::into_raw(Box::new(String::from("nope")))
                }
            }
        }
    }
}