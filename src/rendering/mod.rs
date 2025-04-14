mod shaders;
pub mod camera;
pub mod mesh;
pub mod text;
pub mod render_context;

use crate::RenderContext;
use camera::{Camera, Lens};
use gl;
use glam::Mat4;
use mesh::Mesh;
use sdl2::video::Window;
use shaders::Shader;
use tracing::debug;

pub fn init<'sdl2>(window: &'sdl2 mut Window) -> Renderer<'sdl2> {
    debug!("Initialising a renderer...");
    //Create our shaders 
    //TODO - MAKE THIS SHIT NOT RELATIVE AND BUNDLE WITH RELEASE
    let shader_result = shaders::create_shader("src/rendering/shaders/default.vert", "src/rendering/shaders/default.frag");

    let shader = match shader_result {
        Ok(shader) => shader,
        Err(error) => {
            println!("This is the error: {}", error);
            panic!("{error}")
        },
    };

    let text_shader_result = shaders::create_shader("src/rendering/shaders/default_text.vert", "src/rendering/shaders/default_text.frag");
    let text_shader = match text_shader_result {
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
        text_shader,
        active_lens: lens,
    }
}

pub struct Renderer<'sdl2> {
    window: &'sdl2 mut Window,
    pub shader: Shader,
    pub text_shader: Shader,
    active_lens: Lens,
}

impl<'a> Renderer<'a> {
    pub fn render(&mut self, render_context: RenderContext) {
        unsafe {
            debug!("New frame starting - clearing buffer bit and enabling depth test.");

            // Clear and setup
            gl::ClearColor(0.5, 0.5, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            // Use 3D shader
            debug!("3D rendering beginning, enabling 3D shader.");
            gl::UseProgram(self.shader.shader_program_id);
            let projection_matrix: Mat4 = camera::get_projection_matrix(&self.active_lens);
            let view_matrix: Mat4 = camera::get_view_matrix(render_context.camera);

            let projection_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection_matrix.as_ref().as_ptr());

            let view_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_matrix.as_ref().as_ptr());

            debug!("Rendering all meshes...");
            for render_mesh in render_context.meshes.iter() {
                let model_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"model\0".as_ptr() as *const i8);
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, render_mesh.model.as_ref().as_ptr());

                let color_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"color\0".as_ptr() as *const i8);
                gl::Uniform3f(color_loc, 0.5, 0.0, 0.5);

                render_mesh.mesh.draw();

                // Optional outline rendering
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                gl::LineWidth(1.0);
                gl::Disable(gl::DEPTH_TEST);
                gl::Uniform3f(color_loc, 1.0, 0.0, 1.0);
                render_mesh.mesh.draw();
                gl::Enable(gl::DEPTH_TEST);
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
            debug!("3D rendering finished.");

            // Switch to 2D text rendering
            debug!("2D rendering beginning, enabling 2D shader...");
            gl::Disable(gl::DEPTH_TEST);
            gl::UseProgram(self.text_shader.shader_program_id);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let screen_size = [800.0, 600.0];
            let screen_size_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"screen_size\0".as_ptr() as *const i8);
            gl::Uniform2f(screen_size_loc, screen_size[0], screen_size[1]);

            debug!("Rendering all surfaces...");
            for surface in render_context.quads.iter() {
                let texture = &surface.texture;

                let screen_pos_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"screen_pos\0".as_ptr() as *const i8);
                let scale_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"scale\0".as_ptr() as *const i8);
                let sampler_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"text_texture\0".as_ptr() as *const i8);

                gl::Uniform2f(screen_pos_loc, 0.0, 0.0);
                gl::Uniform2f(scale_loc, texture.width as f32, texture.height as f32);
                gl::Uniform1i(sampler_loc, 0);

                surface.quad.draw_with_texture(texture.texture_id);
            }
            debug!("2D rendering finished.");
        }

        self.window.gl_swap_window();
    }
}
