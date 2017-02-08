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
            //.ok_or(format!("No compatible vertex shader for glsl version {:?}",glsl)));
            .ok_or(format!("No compatible vertex shader")));

        let v_shader_compiled = try!(match
            compile_shader((gl::VERTEX_SHADER), v_shader){
            Ok(id) => Ok(id),
            Err(s) => Err(format!("Error compiling vertex shader: {}", s)),
        });

        let f_shader = try!(fragment_shaders.get(glsl)
            //.ok_or(format!("No compatible fragment shader for glsl version {:?}",glsl)));
            .ok_or(format!("No compatible fragment shader")));


        let f_shader_compiled = try!(match
            compile_shader((gl::FRAGMENT_SHADER), f_shader){
            Ok(id) => Ok(id),
            Err(s) => Err(format!("Error compiling vertex shader: {}", s)),
        });

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
            //.ok_or(format!("No compatible vertex shader for glsl version {:?}",glsl)));
            .ok_or(format!("No compatible vertex shader")));

        let v_shader_compiled = try!(match
            compile_shader((gl::VERTEX_SHADER), v_shader){
            Ok(id) => Ok(id),
            Err(s) => Err(format!("Error compiling vertex shader: {}", s)),
        });

        let f_shader = try!(fragment_shaders.get(glsl)
            //.ok_or(format!("No compatible fragment shader for glsl version {:?}",glsl)));
            .ok_or(format!("No compatible fragment shader")));


        let f_shader_compiled = try!(match
            compile_shader((gl::FRAGMENT_SHADER), f_shader){
            Ok(id) => Ok(id),
            Err(s) => Err(format!("Error compiling vertex shader: {}", s)),
        });

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
        })
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
        }
    }

    /// Sets viewport with normalized coordinates and center as origin.
    pub fn viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            gl::Viewport(x as GLint, y as GLint, w as GLsizei, h as GLsizei);
        }
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
    pub fn draw<F, U>(&mut self, viewport: Viewport, f: F) -> U
        where F: FnOnce(Context, &mut Self) -> U
    {
        let rect = viewport.rect;
        let (x, y, w, h) = (rect[0], rect[1], rect[2], rect[3]);
        self.viewport(x, y, w, h);
        self.clear_program();
        unsafe {
            gl::Enable(gl::FRAMEBUFFER_SRGB);
        }
        let c = Context::new_viewport(viewport);
        let res = f(c, self);
        if self.colored.offset > 0 {
            let program = self.colored.program;
            self.use_program(program);
            self.colored.flush();
        }
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
        }
    }

    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], mut f: F)
        where F: FnMut(&mut FnMut(&[[f32; 2]]))
    {
        let color = gamma_srgb_to_linear(*color);

        // Flush when draw state changes.
        if self.current_draw_state.is_none() ||
           self.current_draw_state.as_ref().unwrap() != draw_state {
            let program = self.colored.program;
            self.use_program(program);
            if self.current_draw_state.is_none() {
                self.use_draw_state(&Default::default());
            }
            self.colored.flush();
            self.use_draw_state(draw_state);
        }

        let ref mut shader = self.colored;
        f(&mut |vertices: &[[f32; 2]]| {
            let items = vertices.len();

            // Render if there is not enough room.
            if shader.offset + items > BUFFER_SIZE * CHUNKS {
                shader.flush();
            }

            for i in 0..items {
                shader.color_buffer[shader.offset + i] = color;
            }
            for i in 0..items {
                shader.pos_buffer[shader.offset + i] = vertices[i];
            }
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

        f(&mut |vertices: &[[f32; 2]], texture_coords: &[[f32; 2]]| {
            unsafe {
                shader.pos.set(vertices);
                shader.uv.set(texture_coords);
                gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
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
