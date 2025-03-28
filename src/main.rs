mod input;
mod rendering;
mod world;
mod debug;

use sdl2::pixels::Color;
use debug::DebugOverlay;
use gl;
use input::{controllers::camera_controller::CameraController, InputAction, InputDispatcher};
use rendering::{text, camera::Camera, RenderCommand};
use world::World;
use glam::{ Mat4, Vec3 };
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

    let world = World::new(); 

    window.set_mouse_grab(true);
    sdl_context.mouse().set_relative_mouse_mode(true);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().show_cursor(false);

    let sdl2_ttf = sdl2::ttf::init().expect("Failed to initialise the sdl2 ttf context!");
    let font = sdl2_ttf.load_font("assets/fonts/FiraCode-SemiBold.tff", 16).expect("Failed to import font.");

    let mut camera = Camera::new();
    let mut debugger = DebugOverlay::new();

    let mut renderer = rendering::init(&mut window, font);
    let event_pump = sdl_context.event_pump().unwrap();
    let mut input_handler = InputDispatcher::new(event_pump);
    let controller = CameraController::new();
    input_handler.set_controller(controller);

    'main: loop {
        let frame_start = std::time::Instant::now();
        let actions = input_handler.poll_events().expect("Error occurred in the input handling loop.");

        for (pos, chunk) in world.chunks.iter() {
            renderer.queue_draw(
                RenderCommand::RenderMesh {
                    mesh: &chunk.mesh,
                    model_matrix: Mat4::from_translation(
                        Vec3::new(
                            pos.0 as f32,
                            pos.1 as f32,
                            pos.2 as f32
                        ) * CHUNK_SIZE as f32 * VOXEL_SIZE
                    )
                }
            );
        }
 
        
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

        let texture = text::create_text_texture(&renderer.font, &text, Color::WHITE);
        let quad = text::new_text_quad();

        renderer.queue_draw(RenderCommand::RenderText { surface: quad, texture });

        for action in actions {
            match action {
                InputAction::Quit => break 'main,
                InputAction::MoveCamera(delta) => camera.move_by(delta),
                InputAction::LookDelta(relative_direction) => camera.apply_look(relative_direction),
            }
        }

        let frame_duration = frame_start.elapsed();
        debugger.update(frame_duration, world.chunks.len(), &camera);
        renderer.render(&camera);
    }

    Ok(())
}

