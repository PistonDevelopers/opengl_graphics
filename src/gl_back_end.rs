//! OpenGL back-end for Rust-Graphics.

// External crates.
use shader_version::{opengl, glsl};
use graphics::BackEnd;
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

static VERTEX_SHADER_XY_RGBA_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;

void main()
{
    gl_Position = pos;
}
";

static VERTEX_SHADER_XY_RGBA_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

in vec4 pos;

void main()
{
    gl_Position = pos;
}
";

static FRAGMENT_SHADER_XY_RGBA_120: &'static str = "
#version 120
uniform vec4 color;

void main()
{
    gl_FragColor = color;
}
";

static FRAGMENT_SHADER_XY_RGBA_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

out vec4 out_color;

void main()
{
    out_color = color;
}
";

static VERTEX_SHADER_XY_RGBA_UV_120: &'static str = "
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

static VERTEX_SHADER_XY_RGBA_UV_150_CORE: &'static str = "
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

static FRAGMENT_SHADER_XY_RGBA_UV_120: &'static str = "
#version 120
uniform vec4 color;
uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    gl_FragColor = texture2D(s_texture, v_uv) * color;
}
";

static FRAGMENT_SHADER_XY_RGBA_UV_150_CORE: &'static str = "
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

struct XYRGBA {
    vao: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    pos: DynamicAttribute,
    color: GLint,
}

impl Drop for XYRGBA {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}

impl XYRGBA {
    fn new(glsl: glsl::GLSL) -> XYRGBA {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            pick_120_150(glsl, VERTEX_SHADER_XY_RGBA_120, VERTEX_SHADER_XY_RGBA_150_CORE)
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            pick_120_150(glsl, FRAGMENT_SHADER_XY_RGBA_120, FRAGMENT_SHADER_XY_RGBA_150_CORE)
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
        XYRGBA {
            vao: vao,
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            pos: pos,
            color: color,
        }
    }
}

struct XYRGBAUV {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    vao: GLuint,
    color: GLint,
    pos: DynamicAttribute,
    uv: DynamicAttribute,
}

impl Drop for XYRGBAUV {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}

impl XYRGBAUV {
    fn new(glsl: glsl::GLSL) -> XYRGBAUV {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            pick_120_150(glsl, VERTEX_SHADER_XY_RGBA_UV_120, VERTEX_SHADER_XY_RGBA_UV_150_CORE)
        ) {
            Ok(id) => id,
            Err(s) => panic!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            pick_120_150(glsl, FRAGMENT_SHADER_XY_RGBA_UV_120, FRAGMENT_SHADER_XY_RGBA_UV_150_CORE)
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
        XYRGBAUV {
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
    xy_rgba: XYRGBA,
    xy_rgba_uv: XYRGBAUV,
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
            xy_rgba: XYRGBA::new(glsl),
            xy_rgba_uv: XYRGBAUV::new(glsl),
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
        unsafe{
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn disable_alpha_blend(&mut self) {
        unsafe{
            gl::Disable(gl::BLEND);
        }
    }

    fn enable_texture(&mut self, texture: &Texture) {
        let texture = texture.get_id();
        unsafe{
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
            let shader_program = self.xy_rgba.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.xy_rgba;

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
            let shader_program = self.xy_rgba_uv.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.xy_rgba_uv;

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

