//! Helper functions for dealing with shaders.

// External crates.
use gl;
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizeiptr, GLuint};
use std::ffi::CString;
use std::{ptr, mem};

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
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl DynamicAttribute {
    /// Binds to a vertex array object.
    ///
    /// The vertex array object remembers the format for later.
    fn bind_vao(&self, vao: GLuint) {
        let stride = 0;
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(self.location,
                                    self.size,
                                    self.ty,
                                    self.normalize,
                                    stride,
                                    ptr::null());
        }
    }

    fn new(program: GLuint,
           name: &str,
           size: i32,
           normalize: GLboolean,
           ty: GLenum,
           vao: GLuint)
           -> Result<Self, String> {
        let location = attribute_location(program, name)?;
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        let res = DynamicAttribute {
            vbo: vbo,
            size: size,
            location: location,
            normalize: normalize,
            ty: ty,
        };
        res.bind_vao(vao);
        Ok(res)
    }

    /// Create XYZ vertex attribute.
    pub fn xyz(program: GLuint, name: &str, vao: GLuint) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 3, gl::FALSE, gl::FLOAT, vao)
    }

    /// Create XY vertex attribute.
    pub fn xy(program: GLuint, name: &str, vao: GLuint) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 2, gl::FALSE, gl::FLOAT, vao)
    }

    /// Create RGB color attribute.
    pub fn rgb(program: GLuint, name: &str, vao: GLuint) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 3, gl::FALSE, gl::FLOAT, vao)
    }

    /// Create RGBA color attribute.
    pub fn rgba(program: GLuint, name: &str, vao: GLuint) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 4, gl::FALSE, gl::FLOAT, vao)
    }

    /// Create texture coordinate attribute.
    pub fn uv(program: GLuint, name: &str, vao: GLuint) -> Result<DynamicAttribute, String> {
        DynamicAttribute::new(program, name, 2, gl::FALSE, gl::FLOAT, vao)
    }

    /// Sets attribute data.
    pub unsafe fn set<T>(&self, data: &[T]) {
        gl::EnableVertexAttribArray(self.location);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       data.len() as GLsizeiptr * mem::size_of::<T>() as GLsizeiptr,
                       mem::transmute(data.as_ptr()),
                       gl::DYNAMIC_DRAW);
    }
}

/// Compiles a shader.
///
/// Returns a shader or a message with the error.
pub fn compile_shader(shader_type: GLenum, source: &str) -> Result<GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_source = match CString::new(source) {
            Ok(x) => x,
            Err(err) => return Err(format!("compile_shader: {}", err)),
        };
        gl::ShaderSource(shader, 1, &c_source.as_ptr(), ptr::null());
        drop(source);
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;

        #[cfg(feature = "glow")]
        {
            gl::GetCompleStatus(shader, &mut status);
        }
        #[cfg(not(feature = "glow"))]
        {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        }

        if status == (gl::TRUE as GLint) {
            Ok(shader)
        } else {
            #[cfg(feature = "glow")]
            {
                Err(gl::GetShaderInfoLog(shader))
            }
            #[cfg(not(feature = "glow"))]
            {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                if len == 0 {
                    Err("Compilation failed with no log. \
                       The OpenGL context might have been created on another thread, \
                       or not have been created."
                        .to_string())
                } else {
                    // Subtract 1 to skip the trailing null character.
                    let mut buf = vec![0; len as usize - 1];
                    gl::GetShaderInfoLog(
                        shader,
                        len,
                        ptr::null_mut(),
                        buf.as_mut_ptr() as *mut GLchar,
                    );

                    gl::DeleteShader(shader);

                    Err(String::from_utf8(buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8"))
                }
            }
        }
    }
}

/// Finds attribute location from a program.
///
/// Returns `Err` if there is no attribute with such name.
pub fn attribute_location(program: GLuint, name: &str) -> Result<GLuint, String> {
    unsafe {
        let c_name = match CString::new(name) {
            Ok(x) => x,
            Err(err) => return Err(format!("attribute_location: {}", err)),
        };
        let id = gl::GetAttribLocation(program, c_name.as_ptr());
        drop(c_name);
        if id < 0 {
            Err(format!("Attribute '{}' does not exists in shader", name))
        } else {
            Ok(id as GLuint)
        }
    }
}

/// Finds uniform location from a program.
///
/// Returns `Err` if there is no uniform with such name.
pub fn uniform_location(program: GLuint, name: &str) -> Result<GLuint, String> {
    unsafe {
        let c_name = match CString::new(name) {
            Ok(x) => x,
            Err(err) => return Err(format!("uniform_location: {}", err)),
        };
        let id = gl::GetUniformLocation(program, c_name.as_ptr());
        drop(c_name);
        if id < 0 {
            Err(format!("Uniform '{}' does not exists in shader", name))
        } else {
            Ok(id as GLuint)
        }
    }
}
