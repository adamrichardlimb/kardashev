mod shaders;

extern crate gl;

use sdl2::{event::Event, keyboard::Keycode, video::Window, EventPump};
use shaders::Shader;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0],[0.5, -0.5, 0.0],[0.0, 0.5, 0.0]];

pub fn init(window: &mut Window) -> Renderer {
    //Create our shaders 
    let shader_result = shaders::create_shader("src/rendering/shaders/vertex_shader.txt", "src/rendering/shaders/fragment_shader.txt");

    println!("{}", shader_result.is_err());

    let shader = match shader_result {
        Ok(shader) => shader,
        Err(error) => {
            println!("This is the error: {}", error);
            panic!("{error}")
        },
    };

    return Renderer {
        window,
        shader
    }
}

pub struct Renderer<'a> {
    window: &'a mut Window,
    shader: Shader,
}

impl Renderer<'_> {
    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ClearColor(0.5, 0.5, 1.0, 1.0);

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

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Show it on the screen
        self.window.gl_swap_window();
    }
}
