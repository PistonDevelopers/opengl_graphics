//! OpenGL back-end for Piston-Graphics.

// External crates.
use std::ffi::CString;
use shader_version::{ OpenGL, Shaders };
use shader_version::glsl::GLSL;
use graphics::{ Context, DrawState, Graphics, Viewport };
use gl;
use gl::types::{
    GLint,
    GLsizei,
    GLuint,
};

// Local crate.
use draw_state;
use Texture;
use shader_utils::{
    compile_shader,
    DynamicAttribute,
};

struct Colored {
    vao: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    pos: DynamicAttribute,
    color: GLint,
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
    fn new(glsl: GLSL) -> Self {
        use shaders::colored;

        let src = |bytes| unsafe { ::std::str::from_utf8_unchecked(bytes) };

        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            Shaders::new().set(GLSL::V1_20, src(colored::VERTEX_GLSL_120))
                          .set(GLSL::V1_50, src(colored::VERTEX_GLSL_150_CORE))
                          .get(glsl).unwrap()
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            Shaders::new().set(GLSL::V1_20, src(colored::FRAGMENT_GLSL_120))
                          .set(GLSL::V1_50, src(colored::FRAGMENT_GLSL_150_CORE))
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

struct Textured {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    vao: GLuint,
    color: GLint,
    pos: DynamicAttribute,
    uv: DynamicAttribute,
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
    fn new(glsl: GLSL) -> Self {
        use shaders::textured;

        let src = |bytes| unsafe { ::std::str::from_utf8_unchecked(bytes) };

        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            Shaders::new().set(GLSL::V1_20, src(textured::VERTEX_GLSL_120))
                          .set(GLSL::V1_50, src(textured::VERTEX_GLSL_150_CORE))
                          .get(glsl).unwrap()
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            Shaders::new().set(GLSL::V1_20, src(textured::FRAGMENT_GLSL_120))
                          .set(GLSL::V1_50, src(textured::FRAGMENT_GLSL_150_CORE))
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

// Newlines and indents for cleaner panic message.
const GL_FUNC_NOT_LOADED: &'static str = "
    OpenGL function pointers must be loaded before creating the `Gl` backend!
    For more info, see the following issue on GitHub:
    https://github.com/PistonDevelopers/opengl_graphics/issues/103
";

/// Contains OpenGL data.
pub struct GlGraphics {
    colored: Colored,
    textured: Textured,
    // Keeps track of the current shader program.
    current_program: Option<GLuint>,
    // Keeps track of the current draw state.
    current_draw_state: Option<DrawState>,
}

impl<'a> GlGraphics {
    /// Creates a new OpenGL back-end.
    ///
    /// # Panics
    /// If the OpenGL function pointers have not been loaded yet.
    /// See https://github.com/PistonDevelopers/opengl_graphics/issues/103 for more info.
    pub fn new(opengl: OpenGL) -> Self {
        assert!(gl::Enable::is_loaded(), GL_FUNC_NOT_LOADED);

        let glsl = opengl.to_glsl();
        // Load the vertices, color and texture coord buffers.
        GlGraphics {
            colored: Colored::new(glsl),
            textured: Textured::new(glsl),
            current_program: None,
            current_draw_state: None,
       }
    }

    /// Sets viewport with normalized coordinates and center as origin.
    pub fn viewport(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32
    ) {
        unsafe {
            gl::Viewport(x as GLint, y as GLint, w as GLsizei, h as GLsizei);
        }
    }

    /// Sets the current program only if the program is not in use.
    pub fn use_program(&mut self, program: GLuint) {
        match self.current_program {
            None => {},
            Some(current_program) => {
                if program == current_program { return }
            },
        }

        unsafe {
            gl::UseProgram(program);
        }
        self.current_program = Some(program);
    }

    /// Unset the current program.
    ///
    /// This forces the current program to be set on next drawing call.
    pub fn clear_program(&mut self) {
        self.current_program = None
    }

    /// Sets the current draw state, by detecting changes.
    pub fn use_draw_state(&mut self, draw_state: &DrawState) {
        match self.current_draw_state {
            None => {
                draw_state::bind_scissor(draw_state.scissor);
                draw_state::bind_stencil(draw_state.stencil);
                draw_state::bind_blend(draw_state.blend);
            }
            Some(ref old_state) => {
                draw_state::bind_state(old_state, draw_state);
            }
        }
        self.current_draw_state = Some(*draw_state);
    }

    /// Unsets the current draw state.
    ///
    /// This forces the current draw state to be set on next drawing call.
    pub fn clear_draw_state(&mut self) {
        self.current_draw_state = None;
    }

    /// Draws graphics.
    pub fn draw<F>(&mut self, viewport: Viewport, f: F)
        where
            F: FnOnce(Context, &mut Self)
    {
        let rect = viewport.rect;
        let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
        self.viewport(x, y, w, h);
        self.clear_program();
        self.enable_alpha_blend();
        let c = Context::new_viewport(viewport);
        f(c, self);
        self.disable_alpha_blend();
    }

    /// Assume all textures has alpha channel for now.
    pub fn has_texture_alpha(&self, _texture: &Texture) -> bool { true }

    /// Enabled alpha blending.
    pub fn enable_alpha_blend(&mut self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    /// Disables alpha blending.
    pub fn disable_alpha_blend(&mut self) {
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }
}

impl Graphics for GlGraphics {
    type Texture = Texture;

    fn clear_color(&mut self, color: [f32; 4]) {
        unsafe {
            let (r, g, b, a) = (color[0], color[1], color[2], color[3]);
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn clear_stencil(&mut self, value: u8) {
        unsafe {
            gl::ClearStencil(value as i32);
        }
    }

    fn tri_list<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        mut f: F
    )
        where F: FnMut(&mut FnMut(&[f32]))
    {
        {
            // Set shader program and draw state.
            let shader_program = self.colored.program;
            self.use_program(shader_program);
            self.use_draw_state(draw_state);
        }
        let ref mut shader = self.colored;

        unsafe {
            gl::BindVertexArray(shader.vao);
            gl::Uniform4f(shader.color, color[0], color[1], color[2], color[3]);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::Disable(gl::CULL_FACE);
        }

        f(&mut |vertices: &[f32]| {
            // xy makes two floats.
            let size_vertices: i32 = 2;
            let items: i32 = vertices.len() as i32 / size_vertices;

            unsafe {
                shader.pos.set(vertices);
                gl::DrawArrays(gl::TRIANGLES, 0, items);
            }
        });

        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &Texture,
        mut f: F
    )
        where F: FnMut(&mut FnMut(&[f32], &[f32]))
    {
        {
            // Set shader program and draw state.
            let shader_program = self.textured.program;
            self.use_program(shader_program);
            self.use_draw_state(draw_state);
        }
        let ref mut shader = self.textured;

        let texture = texture.get_id();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::Disable(gl::CULL_FACE);
            gl::BindVertexArray(shader.vao);
            gl::Uniform4f(shader.color, color[0], color[1], color[2], color[3]);
        }

        f(&mut |vertices: &[f32], texture_coords: &[f32]| {
            let size_vertices: i32 = 2;
            let items: i32 = vertices.len() as i32 / size_vertices;

            unsafe {
                shader.pos.set(vertices);
                shader.uv.set(texture_coords);
                gl::DrawArrays(gl::TRIANGLES, 0, items);
            }
        });

        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

// Might not fail if previous tests loaded functions.
#[test]
#[should_panic]
fn test_gl_loaded() {
    GlGraphics::new(OpenGL::V3_2);
}
