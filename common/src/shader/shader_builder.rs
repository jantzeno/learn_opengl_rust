use crate::shader::shader::Shader;
use core::ffi::CStr;
use glad::gl33::{self as gl, types::*};
use std::{
    ffi::CString,
    fs::{self},
    io::{self},
    ptr,
};

#[derive(Default)]
pub struct ShaderBuilder {
    id: gl::types::GLuint,
}

impl ShaderBuilder {
    pub fn build(&mut self, gl: &mut gl::Gl, vertex_path: &str, fragment_path: &str) -> Shader {
        match self.new_shader(gl, vertex_path, fragment_path) {
            Ok(_) => Shader { id: self.id },
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

    fn new_shader(
        &mut self,
        gl: &mut gl::Gl,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<(), io::Error> {
        // get vertex source file
        let vertex_code = fs::read_to_string(vertex_path)?;
        let vertex_cstring = CString::new(vertex_code)?;
        let vertex_ptr = vertex_cstring.as_ptr();

        // get fragment source file
        let fragment_code = fs::read_to_string(fragment_path)?;
        let fragment_cstring = CString::new(fragment_code)?;
        let fragment_ptr = fragment_cstring.as_ptr();

        // compile shaders
        unsafe {
            // vertex shader
            let vertex = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vertex, 1, &vertex_ptr, ptr::null());
            gl.CompileShader(vertex);
            match self.check_compile_errors(gl, vertex, gl::VERTEX_SHADER) {
                Ok(_) => {}
                Err(e) => {
                    panic!("VERTEX::{}", e);
                }
            }
            // fragment shader
            let fragment = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fragment, 1, &fragment_ptr, ptr::null());
            gl.CompileShader(fragment);
            match self.check_compile_errors(gl, fragment, gl::FRAGMENT_SHADER) {
                Ok(_) => {}
                Err(e) => {
                    panic!("FRAGMENT::{}", e);
                }
            }
            // shader program
            let id = gl.CreateProgram();
            gl.AttachShader(id, vertex);
            gl.AttachShader(id, fragment);
            gl.LinkProgram(id);
            match self.check_compile_errors(gl, id, gl::PROGRAM) {
                Ok(_) => self.id = id,
                Err(e) => {
                    panic!("PROGRAM::{}", e);
                }
            }

            // delete shaders as they're linked into our program now and no longer necessary
            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);
        }

        Ok(())
    }

    fn check_compile_errors(
        &self,
        gl: &mut gl::Gl,
        shader: gl::types::GLuint,
        type_: gl::types::GLenum,
    ) -> Result<(), String> {
        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<i8> = Vec::with_capacity(1024);

        match type_ {
            gl::VERTEX_SHADER | gl::FRAGMENT_SHADER => unsafe {
                gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success == gl::FALSE as GLint {
                    gl.GetShaderInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    Err(format!(
                        "ERROR::SHADER::COMPILATION_FAILED\n{}",
                        CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
                    ))
                } else {
                    Ok(())
                }
            },
            gl::PROGRAM => unsafe {
                gl.GetProgramiv(shader, gl::LINK_STATUS, &mut success);
                if success == gl::FALSE as GLint {
                    gl.GetProgramInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    Err(format!(
                        "ERROR::PROGRAM::LINKING_FAILED\n{}",
                        CStr::from_ptr(info_log.as_ptr()).to_string_lossy()
                    ))
                } else {
                    Ok(())
                }
            },
            _ => Err(String::from("unknown type")),
        }
    }
}
