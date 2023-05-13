use std::ffi::CStr;

use glad::gl33::{self as gl};

pub struct Shader {
    pub id: gl::types::GLuint,
}

impl Shader {
    pub fn use_program(&self, gl: &mut gl::Gl) {
        unsafe { gl.UseProgram(self.id) };
    }

    pub fn set_bool(&self, gl: &mut gl::Gl, name: &CStr, value: bool) {
        unsafe {
            gl.Uniform1i(gl.GetUniformLocation(self.id, name.as_ptr()), value.into());
        }
    }

    pub fn set_int(&self, gl: &mut gl::Gl, name: &CStr, value: i32) {
        unsafe {
            gl.Uniform1i(gl.GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_float(&self, gl: &mut gl::Gl, name: &CStr, value: f32) {
        unsafe {
            gl.Uniform1f(gl.GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }
}
