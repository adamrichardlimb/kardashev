mod shaders;
pub mod camera;
pub mod mesh;
pub mod text;

use text::{TextTexture, TextQuad};
use camera::{Camera, Lens};
use gl;
use glam::Mat4;
use mesh::Mesh;
use sdl2::{surface, ttf::Font, video::Window};
use shaders::Shader;

pub fn init<'sdl2>(window: &'sdl2 mut Window, font: Font<'sdl2, 'sdl2>) -> Renderer<'sdl2, 'sdl2> {
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
        font,
        render_queue: Vec::new()
    }
}

pub enum RenderCommand<'frame> {
    RenderMesh {
        mesh: &'frame Mesh,
        model_matrix: Mat4
    },
    RenderText {
        surface: TextQuad,
        texture: TextTexture
    }
}

impl<'frame> RenderCommand<'frame> {
    pub fn sort_key(&self) -> u8 {
        match self {
            RenderCommand::RenderMesh { .. } => 0,
            RenderCommand::RenderText { .. } => 1,
        }
    }
}

pub struct Renderer<'sdl2, 'frame> {
    window: &'sdl2 mut Window,
    pub shader: Shader,
    pub text_shader: Shader,
    pub font: Font<'sdl2, 'sdl2>,
    active_lens: Lens,
    render_queue: Vec<RenderCommand<'frame>>,
}

impl<'a, 'frame> Renderer<'a, 'frame> {

    pub fn queue_draw(&mut self, command: RenderCommand<'frame>) {
        self.render_queue.push(command);
    }

    pub fn render(&mut self, camera: &Camera) {
        unsafe {
            //Reset all at start of loop
            gl::ClearColor(0.5, 0.5, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            //Start by rendering all meshes
            gl::UseProgram(self.shader.shader_program_id);
            let projection_matrix: Mat4 = camera::get_projection_matrix(&self.active_lens);
            let view_matrix: Mat4 = camera::get_view_matrix(camera);

            //Tell OpenGL to use our matrices
            let projection_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection_matrix.as_ref().as_ptr());
            let view_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_matrix.as_ref().as_ptr());

            for cmd in self.render_queue.iter() {
                if let RenderCommand::RenderMesh { mesh, model_matrix } = cmd {
                    //Set colour of our vertices
                    let color_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"color\0".as_ptr() as *const i8);
                    gl::Uniform3f(color_loc, 0.5, 0.0, 0.5);

                    let model_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"model\0".as_ptr() as *const i8);
                    gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_matrix.as_ref().as_ptr());
                    mesh.draw();

                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    gl::LineWidth(1.0);
                    gl::Disable(gl::DEPTH_TEST);
                    // optional: draw outlines over filled faces
                    gl::Uniform3f(color_loc, 1.0, 0.0, 1.0);
                    mesh.draw();
                    gl::Enable(gl::DEPTH_TEST);
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }

            //Set up for rendering in 2D
            gl::Disable(gl::DEPTH_TEST);
            gl::UseProgram(self.text_shader.shader_program_id);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let screen_size = [800.0, 600.0];
            let screen_size_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"screen_size\0".as_ptr() as *const i8);
            gl::Uniform2f(screen_size_loc, screen_size[0], screen_size[1]);

            for cmd in self.render_queue.iter() {
                if let RenderCommand::RenderText { surface, texture } = cmd {
                   
                    println!("Drawing with texture_id {}", texture.texture_id);

                    // Set screen_pos and scale
                    let screen_pos_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"screen_pos\0".as_ptr() as *const i8);
                    let scale_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"scale\0".as_ptr() as *const i8);
                    gl::Uniform2f(screen_pos_loc, 0.0, 0.0);
                    gl::Uniform2f(scale_loc, texture.width as f32, texture.height as f32);

                    let sampler_loc = gl::GetUniformLocation(self.text_shader.shader_program_id, b"text_texture\0".as_ptr() as *const i8);
                    gl::Uniform1i(sampler_loc, 0);

                    surface.draw_with_texture(texture.texture_id);
                }
            }

            self.render_queue.clear();
        }

        // Show it on the screen
        self.window.gl_swap_window();
    }
}
