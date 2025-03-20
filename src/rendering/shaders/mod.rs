use std::path::Path;
use gl::types::{GLenum, GLuint};
use std::fs::read_to_string;

//TODO
//The idea is you create a shader and get a mutable reference to it
//From there you initialise your chosen shaders
//And then finally link the program to the window
//This then exposes just the methods to create/initialise shaders as well as the Shader struct
//which abstracts away all the OpenGL calls

pub struct Shader {
    shader_program_id: GLuint,
    vertex_shader_id: GLuint,
    fragment_shader_id: GLuint
}

fn initialise_shader<T: AsRef<Path>>(shader_path: T, shader_type: GLenum) -> Result<GLuint, String> {
  unsafe {
  //Match for vertex and fragment shaders
  //Otherwise throw an error
  if shader_type != 0x8B30 && shader_type != 0x8B31 {
      return Err("Shader_type provided is not either a fragment (0x8B30) or vertex shader (0x8B31).".to_string());
  }
  
  let shader_code = read_to_string(shader_path).expect("Unable to read file");

  let shader = gl::CreateShader(shader_type);

  if shader == 0 {
      return Err("An error occurred when attempting to set up a shader with OpenGL, this occurred before importing shader code.".to_string());
  }
  
  gl::ShaderSource(
    shader,
    1,
    &(shader_code.as_bytes().as_ptr().cast()),
    &(shader_code.len().try_into().unwrap())
  );
  
  gl::CompileShader(shader);

  let mut success = 0;
  gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
  
  if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      gl::GetShaderInfoLog(
          shader,
          1024,
          &mut log_len,
          v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      return Err(String::from_utf8_lossy(&v).to_string());
  }

  return Ok(shader);
  }
}

pub fn create_shader<T: AsRef<Path>>(vertex_shader_path: T, fragment_shader_path: T) -> Result<Shader, String> {
    unsafe {
    //First try to initialise our shaders
    let vertex_shader = initialise_shader(vertex_shader_path, 0x8B31);

    let vertex_shader_id = match vertex_shader {
        Ok(id) => id,
        Err(error_string) => return Err(error_string),
    };

    let fragment_shader = initialise_shader(fragment_shader_path, 0x8B30);

    let fragment_shader_id = match fragment_shader {
        Ok(id) => id,
        Err(error_string) => return Err(error_string),
    };
    
    //Create a shader program and save
    let shader_program_id = gl::CreateProgram();

    if shader_program_id == 0 {
        return Err("An error occurred while making the shader program, debug code for this is TODO.".to_string());
    }

    //Link them up
    gl::AttachShader(shader_program_id, vertex_shader_id);
    gl::AttachShader(shader_program_id, fragment_shader_id);
    
    let mut success = 0;
    gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      gl::GetShaderInfoLog(
          shader_program_id,
          1024,
          &mut log_len,
          v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      return Err(String::from_utf8_lossy(&v).to_string());
  }

    gl::DeleteShader(vertex_shader_id);
    gl::DeleteShader(fragment_shader_id);

    gl::UseProgram(shader_program_id);

    return Ok(Shader {
        shader_program_id,
        vertex_shader_id,
        fragment_shader_id
    });
    }
}
