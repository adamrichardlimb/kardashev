use sdl2::ttf::Font;
use sdl2::pixels::Color;

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
