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

const FRAG_SHADER: &str = r#"#version 330 core
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

            //
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);

            println!("{}", vao);

            assert_ne!(vao, 0);
            gl::BindVertexArray(vao);

            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _
            );
            gl::EnableVertexAttribArray(0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Show it on the screen
        self.canvas.present();
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
