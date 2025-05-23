use std::path::Path;
use gl::types::{GLenum, GLuint};
use std::fs::read_to_string;
use regex::Regex;
use tracing::debug;

//TODO
//The idea is you create a shader and get a mutable reference to it
//From there you initialise your chosen shaders
//And then finally link the program to the window
//This then exposes just the methods to create/initialise shaders as well as the Shader struct
//which abstracts away all the OpenGL calls

pub struct Shader {
    pub shader_program_id: GLuint,
    pub vertex_shader_id: GLuint,
    pub fragment_shader_id: GLuint
}

fn initialise_shader(shader_code: &str, shader_type: GLenum) -> Result<GLuint, String> {
  unsafe {
      //Match for vertex and fragment shaders
      //Otherwise throw an error
      if shader_type != 0x8B30 && shader_type != 0x8B31 {
          return Err("Shader_type provided is not either a fragment (0x8B30) or vertex shader (0x8B31).".to_string());
      }

      debug!("Initialising a shader of type {}...", shader_type);

      let shader = gl::CreateShader(shader_type);
      if shader == 0 {
          return Err("An error occurred when attempting to set up a shader with OpenGL, this occurred before importing shader code.".to_string());
      }

      debug!("Shader initialised with OpenGL, attempting to parse shader code and compile shader...");
      
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
          debug!("Shader failed to compile.");
          return Err(String::from_utf8_lossy(&v).to_string());
      }

      debug!("Shader of type {} successfully compiled.", shader_type);

      return Ok(shader);
  }
}

pub fn create_shader<T: AsRef<Path>>(vertex_shader_path: T, fragment_shader_path: T) -> Result<Shader, String> {
    unsafe {
        let vertex_shader_code = read_to_string(vertex_shader_path.as_ref()).expect("Unable to read vertex shader file");
        let attribute_bindings = extract_attribute_bindings(&vertex_shader_code);

        //First try to initialise our shaders
        let vertex_shader = initialise_shader(&vertex_shader_code, 0x8B31);

        let vertex_shader_id = match vertex_shader {
            Ok(id) => id,
            Err(error_string) => return Err(error_string),
        };

        let fragment_shader_code = read_to_string(fragment_shader_path.as_ref()).expect("Unable to read fragment shader file");
        let fragment_shader = initialise_shader(&fragment_shader_code, 0x8B30);

        let fragment_shader_id = match fragment_shader {
            Ok(id) => id,
            Err(error_string) => return Err(error_string),
        };

        debug!("Attempting to create a shader program with OpenGL...");
        
        //Create a shader program and save
        let shader_program_id = gl::CreateProgram();

        if shader_program_id == 0 {
            return Err("An error occurred while making the shader program, debug code for this is TODO.".to_string());
        }

        debug!("Shader program created, with program ID {}, vertex shader id {}, and fragment shader id {}", shader_program_id, vertex_shader_id, fragment_shader_id);
        debug!("Attempting to attach shaders to program...");

        //Link them up
        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);

        for (name, loc) in &attribute_bindings {
            debug!("{}", name);
            //gl::BindAttribLocation(
                //shader_program_id,
                //*loc,
                //format!("{}/0", name).as_ptr() as *const i8
            //);
        }

        gl::LinkProgram(shader_program_id);
        
        debug!("Shaders attached.");

        let mut success = 0;
        gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
        if success == 0 {
          let mut v: Vec<u8> = Vec::with_capacity(1024);
          let mut log_len = 0_i32;
          gl::GetProgramInfoLog(
              shader_program_id,
              1024,
              &mut log_len,
              v.as_mut_ptr().cast(),
          );
          v.set_len(log_len.try_into().unwrap());
          debug!("Link success: {}", success);
          debug!("Log length: {}", log_len);
          return Err(String::from_utf8_lossy(&v).to_string());
    }

    debug!("Shader program compiled successfully.");

    gl::DeleteShader(vertex_shader_id);
    gl::DeleteShader(fragment_shader_id);

    debug!("Shaders deleted now they have been attached.");

    gl::UseProgram(shader_program_id);

    debug!("Shader program now being used.");

    return Ok(Shader {
        shader_program_id,
        vertex_shader_id,
        fragment_shader_id
    });
    }
}

fn extract_attribute_bindings(shader_code: &str) -> Vec<(&str, u32)> {
    let re = Regex::new(r"layout\s*\(\s*location\s*=\s*(\d+)\s*\)\s*in\s+\w+\s+(\w+)\s*;").unwrap();
    
    re.captures_iter(shader_code)
        .map(|cap| {
            let location = cap[1].parse::<u32>().unwrap();
            let name = cap.get(2).unwrap().as_str();
            (name, location)
        })
        .collect()
}
