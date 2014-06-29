//! Helper functions for dealing with shaders.

// External crates.
use gl;
use gl::types::{
    GLboolean,
    GLchar,
    GLenum,
    GLint,
    GLuint,
};

use std::ptr;
use std::mem;

/// Describes a shader attribute.
pub struct DynamicAttribute {
    /// The vertex buffer object.
    vbo: GLuint,
    /// The number of components.
    size: i32,
    /// The location of the attribute in shader.
    location: GLuint,
    /// Whether to normalize when sending to GPU.
    normalize: GLboolean,
    /// The type, for example gl::FLOAT.
    ty: GLenum,
}

impl Drop for DynamicAttribute {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo)
        }
    }
}

impl DynamicAttribute {
    /// Binds to a vertex array object.
    ///
    /// The vertex array object remembers the format for later.
    pub fn bind_vao(&self, vao: GLuint) {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let stride = 0;
        unsafe {
            gl::VertexAttribPointer(
                self.location,
                self.size,
                self.ty,
                self.normalize,
                stride,
                ptr::null()
            );
        }
    }

    fn new(
        program: GLuint, 
        name: &str, 
        size: i32, 
        normalize: GLboolean,
        ty: GLenum
    ) -> Result<DynamicAttribute, String> {
        let location = try!(attribute_location(program, name));
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        Ok(DynamicAttribute {
            vbo: vbo,
            size: size,
            location: location,
            normalize: normalize,
            ty: ty,
        })
    }

    /// Create XYZ vertex attribute.
    pub fn xyz(program: GLuint, name: &str) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 3, gl::FALSE, gl::FLOAT)
    }
    
    /// Create XY vertex attribute.
    pub fn xy(program: GLuint, name: &str) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 2, gl::FALSE, gl::FLOAT)
    }

    /// Create RGB color attribute.
    pub fn rgb(program: GLuint, name: &str) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 3, gl::FALSE, gl::FLOAT)
    }
    
    /// Create RGBA color attribute.
    pub fn rgba(program: GLuint, name: &str) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 4, gl::FALSE, gl::FLOAT)
    }

    /// Create texture coordinate attribute.
    pub fn uv(program: GLuint, name: &str) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 2, gl::FALSE, gl::FLOAT)
    }

    /// Sets attribute data.
    pub unsafe fn set<T>(&self, data: &[T]) {
        gl::EnableVertexAttribArray(self.location);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            data.len() as i64 * mem::size_of::<T>() as i64,
            mem::transmute(data.as_ptr()),
            gl::DYNAMIC_DRAW
        );
    }
}

/// Compiles a shader.
///
/// Returns a shader or a message with the error.
pub fn compile_shader(
    shader_type: GLenum,
    source: &str
) -> Result<GLuint, String> {
    let shader = gl::CreateShader(shader_type);
    unsafe {
        source.with_c_str(
            |ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null())
        );
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == (gl::TRUE as GLint) {
            Ok(shader)
        } else {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            if len == 0 {
                Err("Compilation failed with no log. The OpenGL context might have been created on another thread, or not have been created.".to_string())
            }
            else {
                // Subtract 1 to skip the trailing null character.
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);
                gl::GetShaderInfoLog(
                    shader, 
                    len, 
                    ptr::mut_null(), 
                    buf.as_mut_ptr() as *mut GLchar
                );
                
                gl::DeleteShader(shader);
                
                Err(String::from_utf8(buf).ok().expect(
                    "ShaderInfoLog not valid utf8"
                ))
            }
        }
    }
}

/// Creates a vertex buffer for an attribute from a program.
///
/// Returns `None` if there is no attribute with such name.
pub fn attribute_location(program: GLuint, name: &str) -> Result<GLuint, String> {
    unsafe {
        name.with_c_str(|ptr| {
            let id = gl::GetAttribLocation(program, ptr);
            if id < 0 { 
                Err(format!("Attribute '{}' does not exists in shader", name))
            } else {
                Ok(id as GLuint) 
            }
        })
    }
}

