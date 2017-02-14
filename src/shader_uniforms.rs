//! Types and methods for setting shader uniforms


// External crates.
use std::ffi::CString;
use gl;
use gl::types::{GLboolean, GLint};
use std::marker::PhantomData;

// Local crate.
use back_end::GlGraphics;

/// Describes a shader uniform of a given type.
#[derive(Clone, Copy)]
pub struct ShaderUniform<T : ?Sized>{
    location : GLint,
    phantom : PhantomData<T>,
}

/// Shader uniform type
///
/// For now a small subset
pub trait UniformType {}

/// Shader uniform float
#[derive(Clone, Copy)]
pub struct SUFloat {}
impl UniformType for SUFloat {}

/// Shader uniform integer
#[derive(Clone, Copy)]
pub struct SUInt {}
impl UniformType for SUInt {}

/// Shader uniform vector of size 2
/// Vector elements are floats
#[derive(Clone, Copy)]
pub struct SUVec2 {}
impl UniformType for SUVec2 {}

/// Shader uniform vector of size 3
/// Vector elements are floats
#[derive(Clone, Copy)]
pub struct SUVec3 {}
impl UniformType for SUVec3 {}

/// Shader uniform vector of size 4
/// Vector elements are floats
#[derive(Clone, Copy)]
pub struct SUVec4 {}
impl UniformType for SUVec4 {}

/// Shader uniform 2x2 matrix
/// Matrix elements are floats
#[derive(Clone, Copy)]
pub struct SUMat2x2 {}
impl UniformType for SUMat2x2 {}

/// Shader uniform 3x3 matrix
/// Matrix elements are floats
#[derive(Clone, Copy)]
pub struct SUMat3x3 {}
impl UniformType for SUMat3x3 {}

/// Shader uniform 4x4 matrix
/// Matrix elements are floats
#[derive(Clone, Copy)]
pub struct SUMat4x4 {}
impl UniformType for SUMat4x4 {}

impl GlGraphics {
    /// Try to get uniform from the current shader of a given name.
    pub fn get_uniform<T : UniformType + ?Sized>(&self, name : &str) -> Option<ShaderUniform<T>> {
        self.get_current_program().and_then( |p| {
            unsafe {
                let c_source = CString::new(name).ok();
                c_source.and_then(|name| {
                    let uniform = match gl::GetUniformLocation(p, name.as_ptr()) {
                        -1 => None,
                        location => {
                            Some(ShaderUniform{
                                location : location,
                                phantom : PhantomData,
                            })
                        },
                    };
                    drop(name);
                    uniform
                })
            }
        })
    }
}

impl ShaderUniform<SUFloat> {
    /// Set the value of the float uniform.
    pub fn set(&self, gl : &GlGraphics, value : f32) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform1f(p, self.location, value)}
        });
    }
}

impl ShaderUniform<SUInt> {
    /// Set the value of the integer uniform.
    pub fn set(&self, gl : &GlGraphics, value : i32) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform1i(p, self.location, value)}
        });
    }
}

impl ShaderUniform<SUVec2> {
    /// Set the value of the vector 2 uniform.
    pub fn set(&self, gl : &GlGraphics, value : &[f32; 2]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform2f(p, self.location, value[0], value[1])}
        });
    }
}

impl ShaderUniform<SUVec3> {
    /// Set the value of the vector 3 uniform.
    pub fn set(&self, gl : &GlGraphics, value : &[f32; 3]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform3f(p, self.location, value[0], value[1], value[2])}
        });
    }
}

impl ShaderUniform<SUVec4> {
    /// Set the value of the vector 4 uniform.
    pub fn set(&self, gl : &GlGraphics, value : &[f32; 4]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniform4f(p, self.location, value[0], value[1], value[2], value[3])}
        });
    }
}

impl ShaderUniform<SUMat2x2> {
    /// Set the value of the 2x2 matrix uniform.
    pub fn set(&self, gl : &GlGraphics, values : &[f32; 4]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniformMatrix2fv(p, self.location, 1 as GLint, false as GLboolean, values.as_ptr())}
        });
    }
}

impl ShaderUniform<SUMat3x3> {
    /// Set the value of the 3x3 matrix uniform.
    pub fn set(&self, gl : &GlGraphics, values : &[f32; 9]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniformMatrix3fv(p, self.location, 1 as GLint, false as GLboolean, values.as_ptr())}
        });
    }
}

impl ShaderUniform<SUMat4x4> {
    /// Set the value of the 4x4 matrix uniform.
    pub fn set(&self, gl : &GlGraphics, values : &[f32; 16]) {
        gl.get_current_program().map(|p| {
            unsafe {gl::ProgramUniformMatrix4fv(p, self.location, 1 as GLint, false as GLboolean, values.as_ptr())}
        });
    }
}
