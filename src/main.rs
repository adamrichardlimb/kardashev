mod sdl2_utils;

use gl;
use sdl2_utils::Sdl2Utils;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    let window = video_subsystem
        .window("rust-sdl2 example", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().unwrap();

    //Point our OpenGL calls to SDL2 so they can be fed to the driver
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let event_pump = sdl_context.event_pump()?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    
    let mut utils = Sdl2Utils{
        canvas,
        event_pump,
        gl_context
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

