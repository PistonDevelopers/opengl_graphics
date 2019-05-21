//! OpenGL back-end for Piston-Graphics.

// External crates.
use std::ffi::CString;
use shader_version::{OpenGL, Shaders};
use shader_version::glsl::GLSL;
use graphics::{Context, DrawState, Graphics, Viewport};
use graphics::color::gamma_srgb_to_linear;
use graphics::BACK_END_MAX_VERTEX_COUNT as BUFFER_SIZE;
use gl;
use gl::types::{GLint, GLsizei, GLuint};

// Local crate.
use draw_state;
use Texture;
use shader_utils::{compile_shader, DynamicAttribute};

// The number of chunks to fill up before rendering.
// Amount of memory used: `BUFFER_SIZE * CHUNKS * 4 * (2 + 4)`
// `4` for bytes per f32, and `2 + 4` for position and color.
const CHUNKS: usize = 100;

/// Describes how to render colored objects.
pub struct Colored {
    vao: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    pos: DynamicAttribute,
    color: DynamicAttribute,
    pos_buffer: Vec<[f32; 2]>,
    color_buffer: Vec<[f32; 4]>,
    offset: usize,
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
    /// Generate using pass-through shaders.
    ///
    /// # Panics
    /// If the default pass-through shaders fail to compile
    pub fn new(glsl: GLSL) -> Self {
        use shaders::colored;
        let src = |bytes| unsafe { ::std::str::from_utf8_unchecked(bytes) };

        let mut vertex_shaders = Shaders::new();
        if cfg!(target_os = "emscripten") {
            vertex_shaders
               .set(GLSL::V1_20, src(colored::VERTEX_GLSL_120_WEBGL))
               .set(GLSL::V1_50, src(colored::VERTEX_GLSL_150_CORE_WEBGL))
        } else {
            vertex_shaders
               .set(GLSL::V1_20, src(colored::VERTEX_GLSL_120))
               .set(GLSL::V1_50, src(colored::VERTEX_GLSL_150_CORE))
        };

        let mut fragment_shaders = Shaders::new();
        if cfg!(target_os = "emscripten") {
            fragment_shaders
               .set(GLSL::V1_20, src(colored::FRAGMENT_GLSL_120_WEBGL))
               .set(GLSL::V1_50, src(colored::FRAGMENT_GLSL_150_CORE_WEBGL))
        } else {
           fragment_shaders
               .set(GLSL::V1_20, src(colored::FRAGMENT_GLSL_120))
               .set(GLSL::V1_50, src(colored::FRAGMENT_GLSL_150_CORE))
        };

        Colored::from_vs_fs(glsl, &vertex_shaders, &fragment_shaders).unwrap()
    }

    /// Generate using custom vertex and fragment shaders.
    pub fn from_vs_fs(glsl: GLSL, vertex_shaders   : &Shaders<GLSL, str>,
                                  fragment_shaders : &Shaders<GLSL, str>)
            -> Result<Self, String> {

        let v_shader = try!(vertex_shaders.get(glsl)
            .ok_or("No compatible vertex shader"));

        let v_shader_compiled = try!(
            compile_shader(gl::VERTEX_SHADER, v_shader)
            .map_err(|s| format!("Error compiling vertex shader: {}", s)));

        let f_shader = try!(fragment_shaders.get(glsl)
            .ok_or("No compatible fragment shader"));

        let f_shader_compiled = try!(
            compile_shader(gl::FRAGMENT_SHADER, f_shader)
            .map_err(|s| format!("Error compiling fragment shader: {}", s)));

        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, v_shader_compiled);
            gl::AttachShader(program, f_shader_compiled);

            let c_o_color = CString::new("o_Color").unwrap();
            if cfg!(not(target_os = "emscripten")) {
                gl::BindFragDataLocation(program, 0, c_o_color.as_ptr());
            }
            drop(c_o_color);
        }

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::LinkProgram(program);
        }
        let pos = DynamicAttribute::xy(program, "pos", vao).unwrap();
        let color = DynamicAttribute::rgba(program, "color", vao).unwrap();
        Ok(Colored {
            vao: vao,
            vertex_shader: v_shader_compiled,
            fragment_shader: f_shader_compiled,
            program: program,
            pos: pos,
            color: color,
            pos_buffer: vec![[0.0; 2]; CHUNKS * BUFFER_SIZE],
            color_buffer: vec![[0.0; 4]; CHUNKS * BUFFER_SIZE],
            offset: 0,
        })

    }

    fn flush(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::Disable(gl::CULL_FACE);
            self.color.set(&self.color_buffer[..self.offset]);
            self.pos.set(&self.pos_buffer[..self.offset]);
            gl::DrawArrays(gl::TRIANGLES, 0, self.offset as i32);
            gl::BindVertexArray(0);
        }

        self.offset = 0;
    }
}

/// Describes how to render textured objects.
pub struct Textured {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    vao: GLuint,
    color: GLint,
    pos: DynamicAttribute,
    uv: DynamicAttribute,
    pos_buffer: Vec<[f32; 2]>,
    uv_buffer: Vec<[f32; 2]>,
    offset: usize,
    last_texture_id: GLuint,
    last_color: [f32; 4],
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
    /// Generate using pass-through shaders.
    ///
    /// # Panics
    /// If the default pass-through shaders fail to compile
    pub fn new(glsl: GLSL) -> Self {
        use shaders::textured;
        let src = |bytes| unsafe { ::std::str::from_utf8_unchecked(bytes) };

        let mut vertex_shaders = Shaders::new();
        if cfg!(target_os = "emscripten") {
            vertex_shaders
               .set(GLSL::V1_20, src(textured::VERTEX_GLSL_120_WEBGL))
               .set(GLSL::V1_50, src(textured::VERTEX_GLSL_150_CORE_WEBGL))
        } else {
            vertex_shaders
               .set(GLSL::V1_20, src(textured::VERTEX_GLSL_120))
               .set(GLSL::V1_50, src(textured::VERTEX_GLSL_150_CORE))
        };

        let mut fragment_shaders = Shaders::new();
        if cfg!(target_os = "emscripten") {
            fragment_shaders
               .set(GLSL::V1_20, src(textured::FRAGMENT_GLSL_120_WEBGL))
               .set(GLSL::V1_50, src(textured::FRAGMENT_GLSL_150_CORE_WEBGL))
        } else {
           fragment_shaders
               .set(GLSL::V1_20, src(textured::FRAGMENT_GLSL_120))
               .set(GLSL::V1_50, src(textured::FRAGMENT_GLSL_150_CORE))
        };

        Textured::from_vs_fs(glsl, &vertex_shaders, &fragment_shaders).unwrap()
    }

    /// Generate using custom vertex and fragment shaders.
    pub fn from_vs_fs(glsl: GLSL, vertex_shaders   : &Shaders<GLSL, str>,
                                  fragment_shaders : &Shaders<GLSL, str>)
            -> Result<Self, String> {
        let v_shader = try!(vertex_shaders.get(glsl)
            .ok_or("No compatible vertex shader"));

        let v_shader_compiled = try!(
            compile_shader(gl::VERTEX_SHADER, v_shader)
            .map_err(|s| format!("Error compiling vertex shader: {}", s)));

        let f_shader = try!(fragment_shaders.get(glsl)
            .ok_or("No compatible fragment shader"));

        let f_shader_compiled = try!(
            compile_shader(gl::FRAGMENT_SHADER, f_shader)
            .map_err(|s| format!("Error compiling fragment shader: {}", s)));

        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, v_shader_compiled);
            gl::AttachShader(program, f_shader_compiled);

            let c_o_color = CString::new("o_Color").unwrap();
            if cfg!(not(target_os = "emscripten")) {
                gl::BindFragDataLocation(program, 0, c_o_color.as_ptr());
            }
            drop(c_o_color);
        }

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::LinkProgram(program);
        }
        let pos = DynamicAttribute::xy(program, "pos", vao).unwrap();
        let c_color = CString::new("color").unwrap();
        let color = unsafe { gl::GetUniformLocation(program, c_color.as_ptr()) };
        drop(c_color);
        if color == -1 {
            panic!("Could not find uniform `color`");
        }
        let uv = DynamicAttribute::uv(program, "uv", vao).unwrap();
        Ok(Textured {
            vao: vao,
            vertex_shader: v_shader_compiled,
            fragment_shader: f_shader_compiled,
            program: program,
            pos: pos,
            color: color,
            uv: uv,
            pos_buffer: vec![[0.0; 2]; CHUNKS * BUFFER_SIZE],
            uv_buffer: vec![[0.0; 2]; CHUNKS * BUFFER_SIZE],
            offset: 0,
            last_texture_id: 0,
            last_color: [0.0; 4],
        })
    }

    fn flush(&mut self) {
        let texture_id = self.last_texture_id;
        let color = self.last_color;
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::Uniform4f(self.color, color[0], color[1], color[2], color[3]);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::Disable(gl::CULL_FACE);
            self.pos.set(&self.pos_buffer[..self.offset]);
            self.uv.set(&self.uv_buffer[..self.offset]);
            gl::DrawArrays(gl::TRIANGLES, 0, self.offset as i32);
            gl::BindVertexArray(0);
        }

        self.offset = 0;
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
    // Keeps track of the current viewport
    current_viewport: Option<Viewport>,
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
            current_viewport: None,
        }
    }

    /// Create a new OpenGL back-end with `Colored` and `Textured` structs to describe
    /// how to render objects.
    ///
    /// # Panics
    /// If the OpenGL function pointers have not been loaded yet.
    /// See https://github.com/PistonDevelopers/opengl_graphics/issues/103 for more info.
    pub fn from_colored_textured(colored : Colored, textured : Textured) -> Self {
        assert!(gl::Enable::is_loaded(), GL_FUNC_NOT_LOADED);

        // Load the vertices, color and texture coord buffers.
        GlGraphics {
            colored: colored,
            textured: textured,
            current_program: None,
            current_draw_state: None,
            current_viewport: None,
        }
    }

    /// Sets viewport with normalized coordinates and center as origin.
    fn viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            gl::Viewport(x as GLint, y as GLint, w as GLsizei, h as GLsizei);
        }
    }

    /// Returns the current program
    pub fn get_current_program(&self) -> Option<GLuint> {
        self.current_program
    }

    /// Sets the current program only if the program is not in use.
    pub fn use_program(&mut self, program: GLuint) {
        match self.current_program {
            None => {}
            Some(current_program) => {
                if program == current_program {
                    return;
                }
            }
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
                draw_state::bind_scissor(draw_state.scissor, &self.current_viewport);
                draw_state::bind_stencil(draw_state.stencil);
                draw_state::bind_blend(draw_state.blend);
            }
            Some(ref old_state) => {
                draw_state::bind_state(old_state, draw_state, &self.current_viewport);
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

    /// Setup that should be called at the start of a frame's draw call.
    pub fn draw_begin(&mut self, viewport: Viewport) -> Context {
        let rect = viewport.rect;
        let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
        self.viewport(x, y, w, h);
        self.current_viewport = Some(viewport);
        self.clear_program();
        unsafe {
            gl::Enable(gl::FRAMEBUFFER_SRGB);
        }
        Context::new_viewport(viewport)
    }

    /// Finalize the frame's draw calls.
    pub fn draw_end(&mut self) {
        if self.colored.offset > 0 {
            let program = self.colored.program;
            self.use_program(program);
            self.colored.flush();
        }
        if self.textured.offset > 0 {
            let program = self.textured.program;
            self.use_program(program);
            self.textured.flush();
        }
    }

    /// Convenience for wrapping draw calls with the begin and end methods.
    ///
    /// This is preferred over using the draw_begin & draw_end methods
    /// explicitly but may be less flexible.
    pub fn draw<F, U>(&mut self, viewport: Viewport, f: F) -> U
        where F: FnOnce(Context, &mut Self) -> U
    {
        let c = self.draw_begin(viewport);
        let res = f(c, self);
        self.draw_end();
        res
    }

    /// Assume all textures has alpha channel for now.
    pub fn has_texture_alpha(&self, _texture: &Texture) -> bool {
        true
    }
}

impl Graphics for GlGraphics {
    type Texture = Texture;

    fn clear_color(&mut self, color: [f32; 4]) {
        let color = gamma_srgb_to_linear(color);
        unsafe {
            let (r, g, b, a) = (color[0], color[1], color[2], color[3]);
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn clear_stencil(&mut self, value: u8) {
        unsafe {
            gl::ClearStencil(value as i32);
            gl::Clear(gl::STENCIL_BUFFER_BIT);
        }
    }

    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], mut f: F)
        where F: FnMut(&mut FnMut(&[[f32; 2]]))
    {
        let color = gamma_srgb_to_linear(*color);

        if self.textured.offset > 0 {
            let program = self.textured.program;
            self.use_program(program);
            self.textured.flush();
        }

        // Flush when draw state changes.
        if self.current_draw_state.is_none() ||
           self.current_draw_state.as_ref().unwrap() != draw_state {
            let program = self.colored.program;
            self.use_program(program);
            if self.current_draw_state.is_none() {
                self.use_draw_state(&Default::default());
            }
            if self.colored.offset > 0 {
                self.colored.flush();
            }
            self.use_draw_state(draw_state);
        }

        f(&mut |vertices: &[[f32; 2]]| {
            let items = vertices.len();

            // Render if there is not enough room.
            if self.colored.offset + items > BUFFER_SIZE * CHUNKS {
                let program = self.colored.program;
                self.use_program(program);
                self.colored.flush();
            }

            let ref mut shader = self.colored;
            for i in 0..items {
                shader.color_buffer[shader.offset + i] = color;
            }
            shader.pos_buffer[shader.offset..shader.offset + items]
                  .copy_from_slice(vertices);
            shader.offset += items;
        });
    }

    fn tri_list_uv<F>(&mut self,
                      draw_state: &DrawState,
                      color: &[f32; 4],
                      texture: &Texture,
                      mut f: F)
        where F: FnMut(&mut FnMut(&[[f32; 2]], &[[f32; 2]]))
    {
        let color = gamma_srgb_to_linear(*color);

        if self.colored.offset > 0 {
            let program = self.colored.program;
            self.use_program(program);
            self.colored.flush();
        }

        // Flush when draw state changes.
        if self.current_draw_state.is_none() ||
           self.current_draw_state.as_ref().unwrap() != draw_state ||
           self.textured.last_texture_id != texture.get_id() ||
           self.textured.last_color != color
        {
            let program = self.textured.program;
            if self.current_draw_state.is_none() {
                self.use_draw_state(&Default::default());
            }
            if self.textured.offset > 0 {
                self.use_program(program);
                self.textured.flush();
            }
            self.use_draw_state(draw_state);
        }

        self.textured.last_texture_id = texture.get_id();
        self.textured.last_color = color;
        f(&mut |vertices: &[[f32; 2]], texture_coords: &[[f32; 2]]| {
            let items = vertices.len();

            // Render if there is not enough room.
            if self.textured.offset + items > BUFFER_SIZE * CHUNKS {
                let shader_program = self.textured.program;
                self.use_program(shader_program);
                self.textured.flush();
            }

            let ref mut shader = self.textured;
            shader.pos_buffer[shader.offset..shader.offset + items]
                  .copy_from_slice(vertices);
            shader.uv_buffer[shader.offset..shader.offset + items]
                  .copy_from_slice(texture_coords);
            shader.offset += items;
        });
    }
}

// Might not fail if previous tests loaded functions.
#[test]
#[should_panic]
fn test_gl_loaded() {
    GlGraphics::new(OpenGL::V3_2);
}
