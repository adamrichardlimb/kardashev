use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

mod sdl2_utils;

use sdl2_utils::Sdl2Utils;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    let window = video_subsystem
        .window("rust-sdl2 example", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    
    let mut utils = Sdl2Utils{
        canvas,
        event_pump
    };

    'main: loop {
        
        let events = utils.poll_events();

        if events.is_err() {
            break 'main;
        }

        // Set the background
        utils.render(); 
    }

    Ok(())
}

