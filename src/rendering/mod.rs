mod shaders;
mod camera;

use camera::{Camera, Lens};
use gl;
use glam::Mat4;
use sdl2::video::Window;
use shaders::Shader;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0],[0.5, -0.5, 0.0],[0.0, 0.5, 0.0]];


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

    let camera = Camera::new();
    let lens = Lens::new();

    return Renderer {
        window,
        shader,
        active_camera: camera,
        active_lens: lens
    }
}

pub struct Renderer<'a> {
    window: &'a mut Window,
    shader: Shader,
    active_camera: Camera,
    active_lens: Lens
}

impl Renderer<'_> {
    pub fn render(&self) {
        unsafe {
            
            gl::Enable(gl::DEPTH_TEST);

            let projection_matrix: Mat4 = camera::get_projection_matrix(&self.active_lens);
            
            let view_matrix: Mat4 = camera::get_view_matrix(&self.active_camera);

            let model_matrix: Mat4 = Mat4::IDENTITY;

            //Tell OpenGL to use our matrices
            let projection_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection_matrix.as_ref().as_ptr());

            let view_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_matrix.as_ref().as_ptr());

            let model_loc = gl::GetUniformLocation(self.shader.shader_program_id, b"model\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_matrix.as_ref().as_ptr());

            gl::ClearColor(0.5, 0.5, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
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
