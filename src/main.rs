mod input;
mod rendering;

use gl;
use input::{controllers::camera_controller::CameraController, InputAction, InputDispatcher};
use rendering::camera::Camera;

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
    let event_pump = sdl_context.event_pump().unwrap();

    let mut input_handler = InputDispatcher::new(event_pump);

    let mut camera = Camera::new();
    let controller = CameraController::new();

    input_handler.set_controller(controller);

    'main: loop {
        let actions = input_handler.poll_events().expect("Error occurred in the input handling loop.");
        
        for action in actions {
            match action {
                InputAction::Quit => break 'main,
                InputAction::MoveCamera(delta) => camera.move_by(delta),
            }
        }

        renderer.render(&camera);
    }

    Ok(())
}

