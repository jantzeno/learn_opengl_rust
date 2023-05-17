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
    pub fn set_vec2(&self, gl: &mut gl::Gl, name: &CStr, value: [f32; 2]) {
        unsafe {
            gl.Uniform2fv(
                gl.GetUniformLocation(self.id, name.as_ptr()),
                1,
                value.as_ptr(),
            );
        }
    }
    pub fn sec_vec3(&self, gl: &mut gl::Gl, name: &CStr, value: [f32; 3]) {
        unsafe {
            gl.Uniform3fv(
                gl.GetUniformLocation(self.id, name.as_ptr()),
                1,
                value.as_ptr(),
            );
        }
    }
    pub fn set_vec4(&self, gl: &mut gl::Gl, name: &CStr, value: [f32; 4]) {
        unsafe {
            gl.Uniform4fv(
                gl.GetUniformLocation(self.id, name.as_ptr()),
                1,
                value.as_ptr(),
            );
        }
    }
    pub fn set_mat2(&self, gl: &mut gl::Gl, name: &CStr, mat: [[f32; 2]; 2]) {
        unsafe {
            gl.UniformMatrix2fv(
                gl.GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat[0].as_ptr(),
            )
        }
    }
    pub fn set_mat3(&self, gl: &mut gl::Gl, name: &CStr, mat: [[f32; 3]; 3]) {
        unsafe {
            gl.UniformMatrix2fv(
                gl.GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat[0].as_ptr(),
            )
        }
    }
    pub fn set_mat4(&self, gl: &mut gl::Gl, name: &CStr, mat: [[f32; 4]; 4]) {
        unsafe {
            gl.UniformMatrix2fv(
                gl.GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat[0].as_ptr(),
            )
        }
    }
}
