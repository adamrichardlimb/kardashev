extern crate gl;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::{Window, GLContext }, EventPump};

pub struct Sdl2Utils {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub gl_context: GLContext
}

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0],[0.5, -0.5, 0.0],[0.0, 0.5, 0.0]];

const VERTEX_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAGMENT_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

impl Sdl2Utils {

    pub fn render(&mut self) {
        //Test an OpenGL call
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        self.canvas.clear();

        // Draw a red rectangle
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        
        //Lets see if I can mix the two
        unsafe {

            //Create the VAO and ensure it is assigned
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);

            //All good? Bind it.
            gl::BindVertexArray(vao);

            //Create the VBO and ensure it is assigned
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);

            //All good? Bind it and assign some triangle data.
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            //Tell OpenGL what we will be binding
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _
            );
            gl::EnableVertexAttribArray(0);

            //Now for our shaders
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERTEX_SHADER.as_bytes().as_ptr().cast()),
                &(VERTEX_SHADER.len().try_into().unwrap())
            );
            gl::CompileShader(vertex_shader);
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

            assert_ne!(success, 0);

            //And the fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(FRAGMENT_SHADER.as_bytes().as_ptr().cast()),
                &(FRAGMENT_SHADER.len().try_into().unwrap())
            );
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

            assert_ne!(success, 0);

            //Finally link up the shader program
            let shader_program = gl::CreateProgram();
            assert_ne!(shader_program, 0);
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

            assert_ne!(success, 0);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::UseProgram(shader_program);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Show it on the screen
        self.canvas.window().gl_swap_window();
    }

    pub fn poll_events(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Err("User wishes to close program.".to_string()),
                _ => return Ok(())
            }
        }

        return Ok(());
    }

}
