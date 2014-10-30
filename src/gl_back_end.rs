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
attribute vec4 pos;
attribute vec4 color;

varying vec4 v_color;

void main()
{
    v_color = color;
    gl_Position = pos;
}
";

static VERTEX_SHADER_XY_RGBA_150_CORE: &'static str = "
#version 150 core
in vec4 pos;
in vec4 color;

out vec4 v_color;

void main()
{
    v_color = color;
    gl_Position = pos;
}
";

static FRAGMENT_SHADER_XY_RGBA_120: &'static str = "
#version 120
varying vec4 v_color;

void main()
{
    gl_FragColor = v_color;
}
";

static FRAGMENT_SHADER_XY_RGBA_150_CORE: &'static str = "
#version 150 core
out vec4 out_color;
in vec4 v_color;

void main()
{
    out_color = v_color;
}
";

static VERTEX_SHADER_XY_RGBA_UV_120: &'static str = "
#version 120
attribute vec4 pos;
attribute vec4 color;
attribute vec2 uv;

uniform sampler2D s_texture;

varying vec2 v_uv;
varying vec4 v_color;

void main()
{
    v_uv = uv;
    v_color = color;
    gl_Position = pos;
}
";

static VERTEX_SHADER_XY_RGBA_UV_150_CORE: &'static str = "
#version 150 core
in vec4 pos;
in vec4 color;
in vec2 uv;

uniform sampler2D s_texture;

out vec2 v_uv;
out vec4 v_color;

void main()
{
    v_uv = uv;
    v_color = color;
    gl_Position = pos;
}
";

static FRAGMENT_SHADER_XY_RGBA_UV_120: &'static str = "
#version 120
uniform sampler2D s_texture;

varying vec2 v_uv;
varying vec4 v_color;

void main()
{
    gl_FragColor = texture2D(s_texture, v_uv) * v_color;
}
";

static FRAGMENT_SHADER_XY_RGBA_UV_150_CORE: &'static str = "
#version 150 core
out vec4 out_color;

uniform sampler2D s_texture;

in vec2 v_uv;
in vec4 v_color;

void main()
{
    out_color = texture(s_texture, v_uv) * v_color;
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
    color: DynamicAttribute,
}

impl Drop for XYRGBA {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        gl::DeleteProgram(self.program);
        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);
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
        let program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
            
        unsafe {
            "out_color".with_c_str(
                |ptr| gl::BindFragDataLocation(program, 0, ptr)
            );
        }

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        gl::LinkProgram(program);
        let pos = DynamicAttribute::xy(
                program, 
                "pos", 
                vao
            ).unwrap();
        let color = DynamicAttribute::rgba(
                program, 
                "color", 
                vao
            ).unwrap();
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
    pos: DynamicAttribute,
    color: DynamicAttribute,
    uv: DynamicAttribute,
}

impl Drop for XYRGBAUV {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        gl::DeleteProgram(self.program);
        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);
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

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
       
        unsafe { 
            "out_color".with_c_str(
                |ptr| gl::BindFragDataLocation(program, 0, ptr)
            );
        }
       
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        gl::LinkProgram(program);
        let pos = DynamicAttribute::xy(
                program, 
                "pos", 
                vao
            ).unwrap();
        let color = DynamicAttribute::rgba(
                program, 
                "color", 
                vao
            ).unwrap();
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
        gl::Viewport(x as GLint, y as GLint, w as GLsizei, h as GLsizei);
    }

    /// Sets the current program only if the program is not in use.
    pub fn use_program(&mut self, program: GLuint) {
        match self.current_program {
            None => {},
            Some(current_program) => {
                if program == current_program { return; }
            },
        }

        gl::UseProgram(program);
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
    fn supports_clear_rgba(&self) -> bool { true }

    fn clear_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    fn enable_alpha_blend(&mut self) {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    fn disable_alpha_blend(&mut self) {
        gl::Disable(gl::BLEND);
    }

    fn supports_single_texture(&self) -> bool { true }

    fn enable_single_texture(&mut self, texture: &Texture) {
        let texture = texture.get_id();
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    fn disable_single_texture(&mut self) {}

    // Assume all textures has alpha channel for now.
    fn has_texture_alpha(&self, _texture: &Texture) -> bool { true }

    fn supports_tri_list_xy_f32_rgba_f32(&self) -> bool { true }

    fn tri_list_xy_f32_rgba_f32(
        &mut self,
        vertices: &[f32],
        colors: &[f32]
    ) {
        {
            // Set shader program.
            let shader_program = self.xy_rgba.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.xy_rgba;
        gl::BindVertexArray(shader.vao);

        // xy makes two floats.
        let size_vertices: i32 = 2;

        unsafe {
            shader.pos.set(vertices);
            shader.color.set(colors);
        }
        
        // Render triangles whether they are facing 
        // clockwise or counter clockwise.
        gl::CullFace(gl::FRONT_AND_BACK);

        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::DrawArrays(gl::TRIANGLES, 0, items);
        
        gl::BindVertexArray(0);
    }

    fn supports_tri_list_xy_f32_rgba_f32_uv_f32(&self) -> bool { true }

    fn tri_list_xy_f32_rgba_f32_uv_f32(
        &mut self,
        vertices: &[f32],
        colors: &[f32],
        texture_coords: &[f32]
    ) {
        {
            // Set shader program.
            let shader_program = self.xy_rgba_uv.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.xy_rgba_uv;
        gl::BindVertexArray(shader.vao);
         
        let size_vertices: i32 = 2;
       
        unsafe { 
            shader.pos.set(vertices);
            shader.color.set(colors);
            shader.uv.set(texture_coords);
        }
        
        // Render triangles whether they are facing 
        // clockwise or counter clockwise.
        gl::CullFace(gl::FRONT_AND_BACK);
        
        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::DrawArrays(gl::TRIANGLES, 0, items);

        gl::BindVertexArray(0);
    }
}

