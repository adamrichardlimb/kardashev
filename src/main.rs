mod input;
mod rendering;
mod world;
mod debug;

use debug::DebugOverlay;
use gl;
use input::{controllers::camera_controller::CameraController, InputAction, InputDispatcher};
use rendering::{camera::Camera, RenderCommand};
use world::World;
use glam::{ Mat4, Vec3 };
use world::chunk::{CHUNK_SIZE, VOXEL_SIZE};
use sdl2::pixels::Color;
use rendering::text::{create_text_texture, TextQuad, TextTexture, new_text_quad};

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

    //Create the basic world
    let world = World::new(); 

    //TODO
    //Move this into the camera controller init and some kind of rendering utils
    //Weirdly you need both set_mouse_grab and capture for this to work
    window.set_mouse_grab(true);
    sdl_context.mouse().set_relative_mouse_mode(true);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().show_cursor(false);

    let sdl2_ttf = sdl2::ttf::init().expect("Failed to initialise the sdl2 ttf context!");

    let font = sdl2_ttf.load_font("assets/fonts/FiraCode-SemiBold.tff", 16).expect("Failed to import font.");

    //Okay time for some main loop badness
    let text_quad = new_text_quad();
    let text_texture = create_text_texture(&font, "Hello, world!", Color {r: 0, g: 0, b: 0, a: 1}); 

    //This is our renderer
    let mut renderer = rendering::init(&mut window, font);

    //This is our input and the events it fires
    let event_pump = sdl_context.event_pump().unwrap();

    let mut input_handler = InputDispatcher::new(event_pump);

    let mut camera = Camera::new();
    let controller = CameraController::new();
    
    input_handler.set_controller(controller);

    let mut debugger = DebugOverlay::new();

    'main: loop {
        let frame_start = std::time::Instant::now();

        let actions = input_handler.poll_events().expect("Error occurred in the input handling loop.");

        //Queue the world to render
        for (pos, chunk) in world.chunks.iter() {
            renderer.queue_draw(
                RenderCommand::RenderMesh { mesh: &chunk.mesh,
                model_matrix: Mat4::from_translation(
                    Vec3::new(
                        pos.0 as f32,
                        pos.1 as f32,
                        pos.2 as f32
                    ) * CHUNK_SIZE as f32 * VOXEL_SIZE
                ) }
            );
        }

        
        renderer.queue_draw(RenderCommand::RenderText { surface: &text_quad, texture_id: text_texture.texture_id });

        for action in actions {
            match action {
                InputAction::Quit => break 'main,
                InputAction::MoveCamera(delta) => camera.move_by(delta),
                InputAction::LookDelta(relative_direction) => camera.apply_look(relative_direction),
            }
        }

        let frame_duration = frame_start.elapsed();
        debugger.update(frame_duration, world.chunks.len(), &camera);
        debugger.print();
        renderer.render(&camera);
    }

    Ok(())
}

