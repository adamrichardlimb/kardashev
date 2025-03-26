mod shaders;
pub mod camera;
pub mod mesh;

use camera::{Camera, Lens};
use gl;
use glam::Mat4;
use mesh::Mesh;
use sdl2::video::Window;
use shaders::Shader;

pub fn init(window: &mut Window) -> Renderer {
    //Create our shaders 
    let shader_result = shaders::create_shader("src/rendering/shaders/vertex_shader.txt", "src/rendering/shaders/fragment_shader.txt");

    let shader = match shader_result {
        Ok(shader) => shader,
        Err(error) => {
            println!("This is the error: {}", error);
            panic!("{error}")
        },
    };

    let lens = Lens::new();

    return Renderer {
        window,
        shader,
        active_lens: lens,
        render_queue: Vec::new()
    }
}

pub struct RenderCommand<'frame> {
    pub mesh: &'frame Mesh,
    pub model_matrix: Mat4
}

pub struct Renderer<'a, 'frame> {
    window: &'a mut Window,
    shader: Shader,
    active_lens: Lens,
    render_queue: Vec<RenderCommand<'frame>>,
}

impl<'a, 'frame> Renderer<'a, 'frame> {
    pub fn queue_draw(&mut self, mesh: &'frame Mesh, model_matrix: Mat4) {
        self.render_queue.push(RenderCommand {mesh, model_matrix} );
        println!("Render queue pushed, length {}", self.render_queue.len());
    }

    pub fn render(&self, camera: &Camera) {
        unsafe {
            gl::ClearColor(0.5, 0.5, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            let projection_matrix: Mat4 = camera::get_projection_matrix(&self.active_lens);
            
            let view_matrix: Mat4 = camera::get_view_matrix(camera);

            //Tell OpenGL to use our matrices
            let projection_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection_matrix.as_ref().as_ptr());

            let view_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_matrix.as_ref().as_ptr());

            for cmd in &self.render_queue {            
                let model_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"model\0".as_ptr() as *const i8);
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, cmd.model_matrix.as_ref().as_ptr());
                cmd.mesh.draw();
            }
        }

        // Show it on the screen
        self.window.gl_swap_window();
    }
}
