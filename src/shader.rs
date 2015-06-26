use std::ffi::CString;
use shader_version::Shaders;
use shader_version::glsl::GLSL;
use gl;
use gl::types::{
    GLint,
    GLuint,
};
use shader_utils::{
    compile_shader,
    DynamicAttribute,
};

/// Colored Shader
pub struct Colored {
    pub vao: GLuint,
    pub vertex_shader: GLuint,
    pub fragment_shader: GLuint,
    pub program: GLuint,
    pub pos: DynamicAttribute,
    pub color: GLint,
}

impl Drop for Colored {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}

impl Colored {
    /// Creates a new Colored Shader
    pub fn new(glsl: GLSL) -> Self {
        use shaders::colored;

        let src = |bytes| unsafe { ::std::str::from_utf8_unchecked(bytes) };

        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            Shaders::new().set(GLSL::_1_20, src(colored::VERTEX_GLSL_120))
                          .set(GLSL::_1_50, src(colored::VERTEX_GLSL_150_CORE))
                          .get(glsl).unwrap()
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            Shaders::new().set(GLSL::_1_20, src(colored::FRAGMENT_GLSL_120))
                          .set(GLSL::_1_50, src(colored::FRAGMENT_GLSL_150_CORE))
                          .get(glsl).unwrap()
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };

        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::BindFragDataLocation(program, 0,
                CString::new("o_Color").unwrap().as_ptr());
        }

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::LinkProgram(program);
        }
        let pos = DynamicAttribute::xy(
                program,
                "pos",
                vao
            ).unwrap();
        let color = unsafe {
                gl::GetUniformLocation(program,
                    CString::new("color").unwrap().as_ptr())
            };
        if color == -1 {
            panic!("Could not find uniform `color`");
        }
        Colored {
            vao: vao,
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            pos: pos,
            color: color,
        }
    }
}

/// Textured Shader
pub struct Textured {
    pub vertex_shader: GLuint,
    pub fragment_shader: GLuint,
    pub program: GLuint,
    pub vao: GLuint,
    pub color: GLint,
    pub pos: DynamicAttribute,
    pub uv: DynamicAttribute,
}

impl Drop for Textured {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}

impl Textured {
    /// Creates a new Textured Shader
    pub fn new(glsl: GLSL) -> Self {
        use shaders::textured;

        let src = |bytes| unsafe { ::std::str::from_utf8_unchecked(bytes) };

        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            Shaders::new().set(GLSL::_1_20, src(textured::VERTEX_GLSL_120))
                          .set(GLSL::_1_50, src(textured::VERTEX_GLSL_150_CORE))
                          .get(glsl).unwrap()
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            Shaders::new().set(GLSL::_1_20, src(textured::FRAGMENT_GLSL_120))
                          .set(GLSL::_1_50, src(textured::FRAGMENT_GLSL_150_CORE))
                          .get(glsl).unwrap()
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };

        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::BindFragDataLocation(program, 0,
                CString::new("o_Color").unwrap().as_ptr());
        }

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::LinkProgram(program);
        }
        let pos = DynamicAttribute::xy(
                program,
                "pos",
                vao
            ).unwrap();
        let color = unsafe {
                gl::GetUniformLocation(program,
                    CString::new("color").unwrap().as_ptr())
            };
        if color == -1 {
            panic!("Could not find uniform `color`");
        }
        let uv = DynamicAttribute::uv(
                program,
                "uv",
                vao
            ).unwrap();
        Textured {
            vao: vao,
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            pos: pos,
            color: color,
            uv: uv,
        }
    }
}