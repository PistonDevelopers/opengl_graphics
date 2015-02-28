//! OpenGL back-end for Rust-Graphics.

// External crates.
use std::ffi::CString;
use shader_version::{opengl, glsl};
use graphics::{ Context, Graphics };
use gl;
use gl::types::{
    GLint,
    GLsizei,
    GLuint,
};
use shader_utils::{
    compile_shader,
    DynamicAttribute,
};

// Local crate.
use Texture;

static VS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;

void main()
{
    gl_Position = pos;
}
";

static VS_COLORED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

in vec4 pos;

void main()
{
    gl_Position = pos;
}
";

static FS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

void main()
{
    gl_FragColor = color;
}
";

static FS_COLORED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

out vec4 out_color;

void main()
{
    out_color = color;
}
";

static VS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;
attribute vec2 uv;

uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    v_uv = uv;
    gl_Position = pos;
}
";

static VS_TEXTURED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

in vec4 pos;
in vec2 uv;

uniform sampler2D s_texture;

out vec2 v_uv;

void main()
{
    v_uv = uv;
    gl_Position = pos;
}
";

static FS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;
uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    gl_FragColor = texture2D(s_texture, v_uv) * color;
}
";

static FS_TEXTURED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;
uniform sampler2D s_texture;

out vec4 out_color;

in vec2 v_uv;

void main()
{
    out_color = texture(s_texture, v_uv) * color;
}
";

fn pick_120_150<T>(glsl: glsl::GLSL, for_120: T, for_150: T) -> T {
    use shader_version::glsl::GLSL;
    match glsl {
        GLSL::_1_10 => panic!("GLSL 1.10 not supported"),
        GLSL::_1_20
      | GLSL::_1_30
      | GLSL::_1_40 => for_120,
        _ => for_150,
    }
}

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
    fn new(glsl: glsl::GLSL) -> Colored {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            pick_120_150(glsl, VS_COLORED_120, VS_COLORED_150_CORE)
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            pick_120_150(glsl, FS_COLORED_120, FS_COLORED_150_CORE)
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
                CString::new("out_color".as_bytes()).unwrap().as_ptr());
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
                    CString::new("color".as_bytes()).unwrap().as_ptr())
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
    fn new(glsl: glsl::GLSL) -> Textured {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            pick_120_150(glsl, VS_TEXTURED_120, VS_TEXTURED_150_CORE)
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            pick_120_150(glsl, FS_TEXTURED_120, FS_TEXTURED_150_CORE)
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
                CString::new("out_color".as_bytes()).unwrap().as_ptr());
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
                    CString::new("color".as_bytes()).unwrap().as_ptr())
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
}

impl<'a> GlGraphics {
    /// Creates a new OpenGL back-end.
    ///
    /// # Panics
    /// If the OpenGL function pointers have not been loaded yet.
    /// See https://github.com/PistonDevelopers/opengl_graphics/issues/103 for more info.
    pub fn new(opengl: opengl::OpenGL) -> GlGraphics {
        assert!(gl::Enable::is_loaded(), GL_FUNC_NOT_LOADED);

        let glsl = opengl.to_GLSL();
        // Load the vertices, color and texture coord buffers.
        GlGraphics {
            colored: Colored::new(glsl),
            textured: Textured::new(glsl),
            current_program: None,
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
                if program == current_program { return; }
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

    /// Draws graphics.
    pub fn draw<F>(&mut self, viewport: [i32; 4], f: F)
        where
            F: FnOnce(Context, &mut GlGraphics)
    {
        let [x, y, w, h] = viewport;
        self.viewport(x, y, w, h);
        self.clear_program();
        self.enable_alpha_blend();
        let c = Context::abs(
            w as f64,
            h as f64
        );
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

    fn clear(&mut self, color: [f32; 4]) {
        unsafe {
            let [r, g, b, a] = color;
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn tri_list<F>(&mut self, color: &[f32; 4], mut f: F)
        where F: FnMut(&mut FnMut(&[f32]))
    {
        {
            // Set shader program.
            let shader_program = self.colored.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.colored;

        unsafe {
            gl::BindVertexArray(shader.vao);
            gl::Uniform4f(shader.color, color[0], color[1], color[2], color[3]);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::CullFace(gl::FRONT_AND_BACK);
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

    fn tri_list_uv<F>(&mut self, color: &[f32; 4], texture: &Texture, mut f: F)
        where F: FnMut(&mut FnMut(&[f32], &[f32]))
    {
        {
            // Set shader program.
            let shader_program = self.textured.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.textured;

        let texture = texture.get_id();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::CullFace(gl::FRONT_AND_BACK);
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
#[should_fail]
fn test_gl_loaded() {
    Gl::new(opengl::OpenGL::_3_2);
}
