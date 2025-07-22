// warnings
#![allow(dangerous_implicit_autorefs)]
#![allow(unsafe_op_in_unsafe_fn)]

// imports
use sdl2::{Sdl, VideoSubsystem};
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
    pub image_ctx: *mut Sdl2ImageContext
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
    image_ctx: std::ptr::null_mut()
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
    println!("[butterfly] all done.");

}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_screen() {
    (*SDL2.canvas).set_draw_color(Color::RGB(0, 0, 0));
    (*SDL2.canvas).clear();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn redraw_screen() {
    println!("[butterfly] redraw.");
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