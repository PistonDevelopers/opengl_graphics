//! OpenGL back-end for Piston-Graphics.


// External crates.
use std::ffi::CString;
use gl;
use gl::types::{GLint};
use std::marker::PhantomData;

// Local crate.
use back_end::GlGraphics;

/// TODO
pub trait UniformType {}
/// TODO
pub trait SUFloat : UniformType {}
/// TODO
pub trait SUInt : UniformType {}
/// TODO
pub trait SUVec2 : UniformType {}
/// TODO
pub trait SUVec3 : UniformType {}
/// TODO
pub trait SUVec4 : UniformType {}

/// TODO
#[derive(Clone, Copy)]
pub struct ShaderUniform<T : ?Sized>{
    location : GLint,
    phantom : PhantomData<T>,
}

impl GlGraphics {
/// TODO
    pub fn get_uniform<T : UniformType + ?Sized>(&self, name : &str) -> Option<ShaderUniform<T>> {
        self.get_current_program().and_then( |p| {
            match unsafe {gl::GetUniformLocation(p, CString::new(name).unwrap().
                                                  as_ptr())} {
                -1 => None,
                location => {
                    Some(ShaderUniform{
                        location : location,
                        phantom : PhantomData,
                    })
                },
            }
        })
    }
}

impl ShaderUniform<SUFloat> {
/// TODO
    pub fn set(&self, gl : &GlGraphics, value : f32) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform1f(p, self.location, value)}
        });
    }
}

impl ShaderUniform<SUInt> {
/// TODO
    pub fn set(&self, gl : &GlGraphics, value : i32) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform1i(p, self.location, value)}
        });
    }
}

impl ShaderUniform<SUVec2> {
/// TODO
    pub fn set(&self, gl : &GlGraphics, value : &[f32; 2]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform2f(p, self.location, value[0], value[1])}
        });
    }
}

impl ShaderUniform<SUVec3> {
/// TODO
    pub fn set(&self, gl : &GlGraphics, value : &[f32; 3]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform3f(p, self.location, value[0], value[1], value[2])}
        });
    }
}

impl ShaderUniform<SUVec4> {
/// TODO
    pub fn set(&self, gl : &GlGraphics, value : &[f32; 4]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform4f(p, self.location, value[0], value[1], value[2], value[3])}
        });
    }
}

