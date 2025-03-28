use sdl2::ttf::Font;
use sdl2::pixels::Color;

pub fn new_text_quad() -> TextQuad {
    let vertices: [f32; 16] = [
        // x, y,    u, v
        0.0, 0.0,   0.0, 0.0,
        1.0, 0.0,   1.0, 0.0,
        1.0, 1.0,   1.0, 1.0,
        0.0, 1.0,   0.0, 1.0,
    ];

    let indices: [u32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // Position: 2 floats
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * 4, 0 as *const _);
        gl::EnableVertexAttribArray(0);

        // UV: 2 floats
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * 4, (2 * 4) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
    }

    TextQuad { vao, vbo, ebo }
}

pub struct TextQuad {
    vao: u32,
    vbo: u32,
    ebo: u32
}

impl TextQuad {
    pub fn draw_with_texture(&self, texture_id: u32) {
        unsafe {
            //Bind the texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            //Bind VAO and draw
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

pub struct TextTexture {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32
}

pub fn create_text_texture(font: &Font, text: &str, color: Color) -> TextTexture {
    let surface = font.render(text).blended(color).unwrap();
    let (width, height) = (surface.width(), surface.height());
    let pixels = surface.without_lock().unwrap();

    let mut texture_id = 0;
    unsafe {
        //Create a texture and bind it
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::BGRA,
            gl::UNSIGNED_BYTE,
            pixels.as_ptr() as *const _,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    TextTexture {
        texture_id,
        width,
        height
    }
}
