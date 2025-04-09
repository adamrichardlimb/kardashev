mod input;
mod rendering;
mod world;
mod debug;
mod events;

use std::cell::RefCell;
use std::rc::Rc;
use rendering::render_context::RenderContext;
use rendering::text::Surface2D;
use sdl2::pixels::Color;
use debug::DebugOverlay;
use gl;
use input::{controllers::camera_controller::CameraController, InputAction, InputDispatcher};
use rendering::{text, camera::Camera};
use world::chunk_mesh_manager::{ChunkMeshManager};
use world::World;

use world::chunk::{CHUNK_SIZE, VOXEL_SIZE};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem
        .window("Kardashev", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_queue = events::EventQueue::new();
    let world = Rc::new(RefCell::new(World::new()));

    window.set_mouse_grab(true);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let sdl2_ttf = sdl2::ttf::init().expect("Failed to initialise the sdl2 ttf context!");
    let font = sdl2_ttf.load_font("assets/fonts/FiraCode-SemiBold.tff", 12).expect("Failed to import font.");

    let mut camera = Camera::new();
    let mut debugger = DebugOverlay::new();

    let mut renderer = rendering::init(&mut window);
    let event_pump = sdl_context.event_pump().unwrap();
    let mut input_handler = InputDispatcher::new(event_pump);
    let controller = CameraController::new();
    input_handler.set_controller(controller);

    let chunk_mesh_manager = Rc::new(RefCell::new(ChunkMeshManager::new()));
    let mut chunk_mesh_manager_event_types = Vec::new();
    chunk_mesh_manager_event_types.push(events::EventType::ChunkLoaded);
    chunk_mesh_manager_event_types.push(events::EventType::ChunkUnloaded);
    event_queue.register_listener(chunk_mesh_manager_event_types, chunk_mesh_manager.clone());

    'main: loop {
        let frame_start = std::time::Instant::now();
        let actions = input_handler.poll_events().expect("Error occurred in the input handling loop.");
        world.borrow_mut().update(camera.position(), &mut event_queue);        
        event_queue.dispatch_events();

        let text = format!("Frame: {:.2} ms|Chunks: {}|Draws: {}|Cam: ({:.1}, {:.1}, {:.1}) Yaw: {:.1} Pitch: {:.1}",
            debugger.frame_time_ms,
            debugger.chunk_count,
            debugger.draw_calls,
            debugger.camera_position.x,
            debugger.camera_position.y,
            debugger.camera_position.z,
            debugger.camera_yaw.to_degrees(),
            debugger.camera_pitch.to_degrees()
        );

        let texture = text::create_text_texture(&font, &text, Color::WHITE);
        let quad = text::new_text_quad();

        let fps = Surface2D {quad, texture};

        println!("{}", chunk_mesh_manager.borrow().meshes().len());

        let mut quads = Vec::new();
        quads.push(&fps);

        for action in actions {
            match action {
                InputAction::Quit => break 'main,
                InputAction::MoveCamera(delta) => camera.move_by(delta),
                InputAction::LookDelta(relative_direction) => camera.apply_look(relative_direction),
            }
        }

        let frame_duration = frame_start.elapsed();
        debugger.update(frame_duration, &camera);
      
        let mesh_ref = chunk_mesh_manager.borrow();
        let meshes = mesh_ref.meshes();

        {
            let render_context = RenderContext {
                camera: &camera,
                meshes,
                quads 
            };
            renderer.render(render_context);
        }
    }

    Ok(())
}

