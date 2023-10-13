extern crate sdl2;

mod helper;
mod texture_creator;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use crate::helper::dump::Dump;
use sdl2::render::{TextureCreator};
use helper::color_info::{TextureColor};

const TEXTURE_SIZE: u32 = 64;

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    let window = video_subsystem.window("The rust game", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build().expect("Failed to convert window into canvas");


    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let square_texture = texture_creator::create_texture_rect(&mut canvas, &texture_creator,
                                                              TextureColor::Rose50, TEXTURE_SIZE).expect("Failed to create rect");
    let square_texture_1 = texture_creator::create_texture_rect(&mut canvas, &texture_creator,
                                                                TextureColor::Rose100, TEXTURE_SIZE).expect("Failed to create rect");
    let square_texture_2 = texture_creator::create_texture_rect(&mut canvas, &texture_creator,
                                                                TextureColor::Rose200, TEXTURE_SIZE).expect("Failed to create rect");
    let square_texture_3 = texture_creator::create_texture_rect(&mut canvas, &texture_creator,
                                                                TextureColor::Rose300, TEXTURE_SIZE).expect("Failed to create rect");
    let square_texture_green = texture_creator::create_texture_rect(&mut canvas, &texture_creator,
                                                                    TextureColor::Rose400, TEXTURE_SIZE).expect("Failed to create rect");


    let mut event_pump = sdl_context.event_pump()
        .expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            // event.dump();
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    {
                        "Quitting".dump();
                        break 'running;
                    }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(252, 250, 250));
        canvas.clear();
        canvas.copy(&square_texture, None, Rect::new(400 - 48, 300 - 48, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.copy(&square_texture_1, None, Rect::new(400 - 32, 300 - 32, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.copy(&square_texture_2, None, Rect::new(400 - 16, 300 - 16, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.copy(&square_texture_3, None, Rect::new(400, 300, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.copy(&square_texture_green, None, Rect::new(400 + 16, 300 + 16, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}