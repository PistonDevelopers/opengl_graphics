//! OpenGL back-end for Rust-Graphics.

// External crates.
use shader_version::{opengl, glsl};
use graphics::{ Context, BackEnd };
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
    match glsl {
        glsl::GLSL_1_10 => panic!("GLSL 1.10 not supported"),
        glsl::GLSL_1_20
      | glsl::GLSL_1_30
      | glsl::GLSL_1_40 => for_120,
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

            "out_color".with_c_str(
                |ptr| gl::BindFragDataLocation(program, 0, ptr)
            );
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
                "color".with_c_str(|name| gl::GetUniformLocation(program, name))
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

            "out_color".with_c_str(
                |ptr| gl::BindFragDataLocation(program, 0, ptr)
            );
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
                "color".with_c_str(|name| gl::GetUniformLocation(program, name))
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

/// Contains OpenGL data.
pub struct Gl {
    colored: Colored,
    textured: Textured,
    // Keeps track of the current shader program.
    current_program: Option<GLuint>,
    color: [f32, ..4],
}


impl<'a> Gl {
    /// Creates a new OpenGl back-end.
    pub fn new(opengl: opengl::OpenGL) -> Gl {
        let glsl = opengl.to_GLSL();
        // Load the vertices, color and texture coord buffers.
        Gl {
            colored: Colored::new(glsl),
            textured: Textured::new(glsl),
            current_program: None,
            color: [1.0, ..4],
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
    pub fn draw(&mut self, viewport: [i32, ..4], f: |c: Context, g: &mut Gl|) {
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
}

impl BackEnd<Texture> for Gl {
    fn clear(&mut self, color: [f32, ..4]) {
        unsafe {
            let [r, g, b, a] = color;
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn enable_alpha_blend(&mut self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn disable_alpha_blend(&mut self) {
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }

    fn enable_texture(&mut self, texture: &Texture) {
        let texture = texture.get_id();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }
    }

    fn disable_texture(&mut self) {}

    // Assume all textures has alpha channel for now.
    fn has_texture_alpha(&self, _texture: &Texture) -> bool { true }

    fn color(&mut self, color: [f32, ..4]) {
        self.color = color;
    }

    fn tri_list(&mut self, vertices: &[f32]) {
        {
            // Set shader program.
            let shader_program = self.colored.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.colored;

        // xy makes two floats.
        let size_vertices: i32 = 2;
        let items: i32 = vertices.len() as i32 / size_vertices;

        let color = self.color;
        unsafe {
            gl::BindVertexArray(shader.vao);
            shader.pos.set(vertices);
            gl::Uniform4f(shader.color, color[0], color[1], color[2], color[3]);

            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::CullFace(gl::FRONT_AND_BACK);

            gl::DrawArrays(gl::TRIANGLES, 0, items);

            gl::BindVertexArray(0);
        }
    }

    fn tri_list_uv(&mut self, vertices: &[f32], texture_coords: &[f32]) {
        {
            // Set shader program.
            let shader_program = self.textured.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.textured;

        let size_vertices: i32 = 2;
        let items: i32 = vertices.len() as i32 / size_vertices;

        let color = self.color;
        unsafe {
            gl::BindVertexArray(shader.vao);
            shader.pos.set(vertices);
            gl::Uniform4f(shader.color, color[0], color[1], color[2], color[3]);
            shader.uv.set(texture_coords);
            // Render triangles whether they are facing
            // clockwise or counter clockwise.
            gl::CullFace(gl::FRONT_AND_BACK);

            gl::DrawArrays(gl::TRIANGLES, 0, items);

            gl::BindVertexArray(0);
        }
    }
}

