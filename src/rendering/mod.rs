mod shaders;
pub mod camera;

use camera::{Camera, Lens};
use gl;
use glam::Mat4;
use sdl2::video::Window;
use shaders::Shader;
use std::time::Instant;

const CUBE_VERTICES: &[f32] = &[
    -0.5, -0.5, -0.5, //Back, bottom left
     0.5, -0.5, -0.5, //Back, bottom right
     0.5,  0.5, -0.5, //Back, top right
    -0.5,  0.5, -0.5, //Back, top left
    -0.5, -0.5,  0.5, //Front, bottom left
     0.5, -0.5,  0.5, //Front, bottom right
     0.5,  0.5,  0.5, //Front, top right
    -0.5,  0.5,  0.5  //Front, top left
];

const CUBE_INDICES: &[u32] = &[
    0,1,2,2,3,0, //Back
    4,5,6,6,7,4, //Front
    4,0,3,3,7,4, //Left
    1,5,6,6,2,1, //Right
    3,2,6,6,7,3, //Top
    0,1,5,5,4,0, //Bottom
];

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
    let start_time = Instant::now();

    return Renderer {
        window,
        shader,
        active_lens: lens,
        start_time
    }
}

pub struct Renderer<'a> {
    window: &'a mut Window,
    shader: Shader,
    active_lens: Lens,
    start_time: Instant
}

impl Renderer<'_> {
    pub fn render(&self, camera: &Camera) {
        unsafe {
            
            gl::Enable(gl::DEPTH_TEST);

            let projection_matrix: Mat4 = camera::get_projection_matrix(&self.active_lens);
            
            let view_matrix: Mat4 = camera::get_view_matrix(camera);

            let angle = self.start_time.elapsed().as_secs_f32(); // seconds
            let model_matrix = Mat4::from_rotation_y(angle);

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
                (CUBE_VERTICES.len() * std::mem::size_of::<f32>()) as isize,
                CUBE_VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (CUBE_INDICES.len() * std::mem::size_of::<u32>()) as isize,
                CUBE_INDICES.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            //Tell OpenGL what we will be binding
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * 4,
                0 as *const _
            );
            gl::EnableVertexAttribArray(0); 

            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, std::ptr::null());
        }

        // Show it on the screen
        self.window.gl_swap_window();
    }
}
