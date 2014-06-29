//! OpenGL back-end for Rust-Graphics.

// External crates.
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

static VERTEX_SHADER_TRI_LIST_XY_RGBA: &'static str = "
#version 330
in vec4 a_v4Position;
in vec4 a_v4FillColor;

out vec4 v_v4FillColor;

void main()
{
    v_v4FillColor = a_v4FillColor;
    gl_Position = a_v4Position;
}
";

static FRAGMENT_SHADER_TRI_LIST_XY_RGBA: &'static str = "
#version 330
out vec4 out_color;
in vec4 v_v4FillColor;

void main()
{
    out_color = v_v4FillColor;
}
";

static VERTEX_SHADER_TRI_LIST_XY_RGBA_UV: &'static str = "
#version 330
in vec4 a_v4Position;
in vec4 a_v4FillColor;
in vec2 a_v2TexCoord;

uniform sampler2D s_texture;

out vec2 v_v2TexCoord;
out vec4 v_v4FillColor;

void main()
{
    v_v2TexCoord = a_v2TexCoord;
    v_v4FillColor = a_v4FillColor;
    gl_Position = a_v4Position;
}
";

static FRAGMENT_SHADER_TRI_LIST_XY_RGBA_UV: &'static str = "
#version 330
out vec4 out_color;

uniform sampler2D s_texture;

in vec2 v_v2TexCoord;
in vec4 v_v4FillColor;

void main()
{
    out_color = texture(s_texture, v_v2TexCoord) * v_v4FillColor;
}
";

struct TriListXYRGBA {
    vao: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    a_v4Position: DynamicAttribute,
    a_v4FillColor: DynamicAttribute,
}

impl Drop for TriListXYRGBA {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        gl::DeleteProgram(self.program);
        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);
    }
}

impl TriListXYRGBA {
    fn new() -> TriListXYRGBA {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            VERTEX_SHADER_TRI_LIST_XY_RGBA      // shader source
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            FRAGMENT_SHADER_TRI_LIST_XY_RGBA    // shader source
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
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
        let a_v4Position = DynamicAttribute::xy(program, "a_v4Position").unwrap();
        let a_v4FillColor = DynamicAttribute::rgba(program, "a_v4FillColor").unwrap();
        a_v4Position.bind_vao(vao);
        a_v4FillColor.bind_vao(vao);
        TriListXYRGBA {
            vao: vao,
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            a_v4Position: a_v4Position,
            a_v4FillColor: a_v4FillColor,
        }
    }
}

struct TriListXYRGBAUV {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
    vao: GLuint,
    a_v4Position: DynamicAttribute,
    a_v4FillColor: DynamicAttribute,
    a_v2TexCoord: DynamicAttribute,
}

impl Drop for TriListXYRGBAUV {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        gl::DeleteProgram(self.program);
        gl::DeleteShader(self.vertex_shader);
        gl::DeleteShader(self.fragment_shader);
    }
}

impl TriListXYRGBAUV {
    fn new() -> TriListXYRGBAUV {
        let vertex_shader = match compile_shader(
            gl::VERTEX_SHADER,                  // shader type
            VERTEX_SHADER_TRI_LIST_XY_RGBA_UV   // shader type
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
        };
        let fragment_shader = match compile_shader(
            gl::FRAGMENT_SHADER,                // shader type
            FRAGMENT_SHADER_TRI_LIST_XY_RGBA_UV // shader source
        ) {
            Ok(id) => id,
            Err(s) => fail!("compile_shader: {}", s)
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
        let a_v4Position = DynamicAttribute::xy(program, "a_v4Position").unwrap();
        let a_v4FillColor = DynamicAttribute::rgba(program, "a_v4FillColor").unwrap();
        let a_v2TexCoord = DynamicAttribute::uv(program, "a_v2TexCoord").unwrap();
        a_v4Position.bind_vao(vao);
        a_v4FillColor.bind_vao(vao);
        a_v2TexCoord.bind_vao(vao);
        TriListXYRGBAUV {
            vao: vao,
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            a_v4Position: a_v4Position,
            a_v4FillColor: a_v4FillColor,
            a_v2TexCoord: a_v2TexCoord,
        }
    }
}

/// Contains OpenGL data.
pub struct Gl {
    tri_list_xy_rgba: TriListXYRGBA,
    tri_list_xy_rgba_uv: TriListXYRGBAUV,
    // Keeps track of the current shader program.
    current_program: Option<GLuint>,
}


impl<'a> Gl {
    /// Creates a new OpenGl back-end.
    pub fn new() -> Gl {
        // Load the vertices, color and texture coord buffers.
        Gl {
            tri_list_xy_rgba: TriListXYRGBA::new(),
            tri_list_xy_rgba_uv: TriListXYRGBAUV::new(),
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
            let shader_program = self.tri_list_xy_rgba.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.tri_list_xy_rgba;
        gl::BindVertexArray(shader.vao);

        // xy makes two floats.
        let size_vertices: i32 = 2;

        unsafe {
            shader.a_v4Position.set(vertices);
            shader.a_v4FillColor.set(colors);
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
            let shader_program = self.tri_list_xy_rgba_uv.program;
            self.use_program(shader_program);
        }
        let ref mut shader = self.tri_list_xy_rgba_uv;
        gl::BindVertexArray(shader.vao);
         
        let size_vertices: i32 = 2;
       
        unsafe { 
            shader.a_v4Position.set(vertices);
            shader.a_v4FillColor.set(colors);
            shader.a_v2TexCoord.set(texture_coords);
        }
        
        // Render triangles whether they are facing 
        // clockwise or counter clockwise.
        gl::CullFace(gl::FRONT_AND_BACK);
        
        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::DrawArrays(gl::TRIANGLES, 0, items);

        gl::BindVertexArray(0);
    }
}

