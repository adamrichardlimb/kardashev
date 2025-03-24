mod input;
mod rendering;

use gl;

pub fn main() -> Result<(), String> {

    //SDL2 creates a window and OpenGL Context for us
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    //The window where everything happens
    let mut window = video_subsystem
        .window("Kardashev", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    //Create an OpenGL context
    let _gl_context = window.gl_create_context().unwrap();

    //Point our OpenGL calls to SDL2 so they can be fed to the driver
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //This is our renderer
    let renderer = rendering::init(&mut window);

    //This is our input and the events it fires
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut event_subsystem = sdl_context.event().unwrap();

    let mut input_handler = input::init(event_pump, event_subsystem);

    'main: loop {
        let events = input_handler.poll_events();
        
        if events.is_err() {
            break 'main;
        }

        renderer.render();
    }

    Ok(())
}

