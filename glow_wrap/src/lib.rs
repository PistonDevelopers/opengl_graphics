use std::ffi::CStr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::sync::OnceLock;

use glow::HasContext;
use glow::NativeBuffer;
use glow::NativeProgram;
use glow::NativeShader;
use glow::NativeUniformLocation;
use glow::NativeVertexArray;

static CONTEXT: OnceLock<Arc<glow::Context>> = OnceLock::new();

pub fn init(ctx: Arc<glow::Context>) {
    let _ = CONTEXT.set(ctx);
}

pub mod types {
    use std::usize;

    pub type GLfloat = f32;
    pub type GLenum = u32;
    pub type GLuint = u32;
    pub type GLsizei = i32;
    pub type GLboolean = bool;
    pub type GLint = i32;
    pub type GLchar = std::ffi::c_char;
    pub type GLsizeiptr = usize;
    pub type GLbitfield = u32;
}

pub const LINEAR: u32 = glow::LINEAR;
pub const NEAREST: u32 = glow::NEAREST;
pub const LINEAR_MIPMAP_LINEAR: u32 = glow::LINEAR_MIPMAP_LINEAR;
pub const LINEAR_MIPMAP_NEAREST: u32 = glow::LINEAR_MIPMAP_NEAREST;
pub const NEAREST_MIPMAP_LINEAR: u32 = glow::NEAREST_MIPMAP_LINEAR;
pub const NEAREST_MIPMAP_NEAREST: u32 = glow::NEAREST_MIPMAP_NEAREST;
pub const REPEAT: u32 = glow::REPEAT;
pub const MIRRORED_REPEAT: u32 = glow::MIRRORED_REPEAT;
pub const CLAMP_TO_EDGE: u32 = glow::CLAMP_TO_EDGE;
pub const CLAMP_TO_BORDER: u32 = glow::CLAMP_TO_BORDER;
pub const RGBA: u32 = glow::RGBA;
pub const SRGB_ALPHA: u32 = glow::SRGB_ALPHA;

pub const TEXTURE_2D: u32 = glow::TEXTURE_2D;
pub const TEXTURE_MIN_FILTER: u32 = glow::TEXTURE_MIN_FILTER;
pub const TEXTURE_MAG_FILTER: u32 = glow::TEXTURE_MAG_FILTER;
pub const TEXTURE_WRAP_S: u32 = glow::TEXTURE_WRAP_S;
pub const TEXTURE_WRAP_T: u32 = glow::TEXTURE_WRAP_T;
pub const TEXTURE_BORDER_COLOR: u32 = glow::TEXTURE_BORDER_COLOR;

pub const UNSIGNED_BYTE: u32 = glow::UNSIGNED_BYTE;
pub const ARRAY_BUFFER: u32 = glow::ARRAY_BUFFER;
pub const DYNAMIC_DRAW: u32 = glow::DYNAMIC_DRAW;
pub const COMPILE_STATUS: u32 = glow::COMPILE_STATUS;
pub const CULL_FACE: u32 = glow::CULL_FACE;
pub const TRIANGLES: u32 = glow::TRIANGLES;
pub const VERTEX_SHADER: u32 = glow::VERTEX_SHADER;
pub const FRAGMENT_SHADER: u32 = glow::FRAGMENT_SHADER;
pub const FRAMEBUFFER_SRGB: u32 = glow::FRAMEBUFFER_SRGB;
pub const COLOR_BUFFER_BIT: u32 = glow::COLOR_BUFFER_BIT;
pub const DEPTH_BUFFER_BIT: u32 = glow::DEPTH_BUFFER_BIT;
pub const STENCIL_BUFFER_BIT: u32 = glow::STENCIL_BUFFER_BIT;
pub const SCISSOR_TEST: u32 = glow::SCISSOR_TEST;
pub const STENCIL_TEST: u32 = glow::STENCIL_TEST;

pub const INCR: u32 = glow::INCR;
pub const KEEP: u32 = glow::KEEP;
pub const REPLACE: u32 = glow::REPLACE;
pub const EQUAL: u32 = glow::EQUAL;
pub const NOTEQUAL: u32 = glow::NOTEQUAL;

pub const FUNC_ADD: u32 = glow::FUNC_ADD;
pub const SRC_ALPHA: u32 = glow::SRC_ALPHA;
pub const ONE_MINUS_SRC_ALPHA: u32 = glow::ONE_MINUS_SRC_ALPHA;
pub const ONE: u32 = glow::ONE;
pub const ZERO: u32 = glow::ZERO;
pub const DST_COLOR: u32 = glow::DST_COLOR;
pub const DST_ALPHA: u32 = glow::DST_ALPHA;
pub const FUNC_SUBTRACT: u32 = glow::FUNC_SUBTRACT;
pub const CONSTANT_COLOR: u32 = glow::CONSTANT_COLOR;
pub const SRC_COLOR: u32 = glow::SRC_COLOR;

pub const BLEND: u32 = glow::BLEND;

pub const NEVER: u32 = glow::NEVER;
pub const TRUE: bool = true;
pub const FALSE: bool = false;
pub const FLOAT: u32 = glow::FLOAT;

#[inline]
fn gl() -> &'static glow::Context {
    CONTEXT.get().unwrap().as_ref()
}

#[allow(non_snake_case)]
pub unsafe fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) {
    let textures = std::slice::from_raw_parts(textures, n as usize);
    for texture in textures {
        gl().delete_texture(glow::NativeTexture(NonZeroU32::new_unchecked(*texture)));
    }
}

#[allow(non_snake_case)]
pub unsafe fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) {
    if let Ok(id) = gl().create_texture() {
        *textures = id.0.get();
    }
}

#[allow(non_snake_case)]
pub unsafe fn BindTexture(target: u32, texture: types::GLuint) {
    gl().bind_texture(
        target,
        Some(glow::NativeTexture(NonZeroU32::new_unchecked(texture))),
    );
}

#[allow(non_snake_case)]
pub unsafe fn TexParameteri(target: u32, parameter: u32, value: i32) {
    gl().tex_parameter_i32(target, parameter, value);
}

#[allow(non_snake_case)]
pub unsafe fn TexParameterfv(target: u32, parameter: u32, values: *const f32) {
    let values = std::slice::from_raw_parts(values, 4);
    gl().tex_parameter_f32_slice(target, parameter, values);
}

#[allow(non_snake_case)]
pub unsafe fn GenerateMipmap(target: u32) {
    gl().generate_mipmap(target);
}

#[allow(non_snake_case)]
pub unsafe fn TexImage2D(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    border: i32,
    format: u32,
    ty: u32,
    pixels: *const u8,
) {
    let pixels = std::slice::from_raw_parts(pixels, 4);
    gl().tex_image_2d(
        target,
        level,
        internal_format,
        width,
        height,
        border,
        format,
        ty,
        Some(pixels),
    );
}

#[allow(non_snake_case)]
pub unsafe fn TexSubImage2D(
    target: u32,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    border: i32,
    format: u32,
    ty: u32,
    pixels: *const u8,
) {
    let pixels = std::slice::from_raw_parts(pixels, 4);
    gl().tex_sub_image_2d(
        target,
        level,
        internal_format,
        width,
        height,
        border,
        format,
        ty,
        glow::PixelUnpackData::Slice(pixels),
    );
}

#[allow(non_snake_case)]
pub unsafe fn GetUniformLocation(
    program: types::GLuint,
    name: *const types::GLchar,
) -> types::GLint {
    let name = char_ptr_to_str(name);
    let location =
        gl().get_uniform_location(NativeProgram(NonZeroU32::new_unchecked(program)), name);
    if let Some(location) = location {
        return location.0 as types::GLint;
    }
    return 0;
}

#[allow(non_snake_case)]
pub unsafe fn CreateProgram() -> types::GLuint {
    match gl().create_program() {
        Ok(program) => return program.0.get(),
        Err(_) => {}
    };
    return 0;
}

#[allow(non_snake_case)]
pub unsafe fn LinkProgram(program: types::GLuint) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    gl().link_program(program);
}

#[allow(non_snake_case)]
pub unsafe fn UseProgram(program: types::GLuint) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    gl().use_program(Some(program));
}

#[allow(non_snake_case)]
pub unsafe fn DeleteProgram(program: types::GLuint) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    gl().delete_program(program);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform1f(program: types::GLuint, location: types::GLint, value: f32) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    gl().uniform_1_f32(Some(&location), value);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform1i(program: types::GLuint, location: types::GLint, value: i32) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    gl().uniform_1_i32(Some(&location), value);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform2f(program: types::GLuint, location: types::GLint, x: f32, y: f32) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    gl().uniform_2_f32(Some(&location), x, y);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform3f(
    program: types::GLuint,
    location: types::GLint,
    x: f32,
    y: f32,
    z: f32,
) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    gl().uniform_3_f32(Some(&location), x, y, z);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform4f(
    program: types::GLuint,
    location: types::GLint,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    gl().uniform_4_f32(Some(&location), x, y, z, w);
}

#[allow(non_snake_case)]
pub unsafe fn Uniform4f(location: types::GLint, x: f32, y: f32, z: f32, w: f32) {
    let location = NativeUniformLocation(location as u32);
    gl().uniform_4_f32(Some(&location), x, y, z, w);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniformMatrix2fv(
    program: types::GLuint,
    location: types::GLint,
    count: types::GLint,
    transpose: types::GLboolean,
    value: *const f32,
) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    let value = std::slice::from_raw_parts(value, count as usize * 4);
    gl().uniform_matrix_2_f32_slice(Some(&location), transpose, value);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniformMatrix3fv(
    program: types::GLuint,
    location: types::GLint,
    count: types::GLint,
    transpose: types::GLboolean,
    value: *const f32,
) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    let value = std::slice::from_raw_parts(value, count as usize * 9);
    gl().uniform_matrix_3_f32_slice(Some(&location), transpose, value);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniformMatrix4fv(
    program: types::GLuint,
    location: types::GLint,
    count: types::GLint,
    transpose: types::GLboolean,
    value: *const f32,
) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let location = NativeUniformLocation(location as u32);
    gl().use_program(Some(program));
    let value = std::slice::from_raw_parts(value, count as usize * 16);
    gl().uniform_matrix_4_f32_slice(Some(&location), transpose, value);
}

#[allow(non_snake_case)]
pub unsafe fn DeleteBuffers(_: types::GLsizei, buffer: &types::GLuint) {
    let buffer = NativeBuffer(NonZeroU32::new_unchecked(*buffer));
    gl().delete_buffer(buffer)
}

#[allow(non_snake_case)]
pub unsafe fn BindVertexArray(array: types::GLuint) {
    let array = NativeVertexArray(NonZeroU32::new_unchecked(array));
    gl().bind_vertex_array(Some(array));
}

#[allow(non_snake_case)]
pub unsafe fn GenVertexArrays(_: types::GLsizei, arrays: *mut types::GLuint) {
    match gl().create_vertex_array() {
        Ok(array) => *arrays = array.0.get(),
        Err(_) => {}
    };
}

#[allow(non_snake_case)]
pub unsafe fn DeleteVertexArrays(_: types::GLsizei, array: *const types::GLuint) {
    let array = NativeVertexArray(NonZeroU32::new_unchecked(*array));
    gl().delete_vertex_array(array);
}

#[allow(non_snake_case)]
pub unsafe fn BindBuffer(target: types::GLenum, buffer: types::GLuint) {
    let buffer = NativeBuffer(NonZeroU32::new_unchecked(buffer));
    gl().bind_buffer(target, Some(buffer));
}

#[allow(non_snake_case)]
pub unsafe fn VertexAttribPointer(
    index: types::GLuint,
    size: types::GLint,
    data_type: types::GLenum,
    normalized: types::GLboolean,
    stride: types::GLint,
    offset: *const types::GLint,
) {
    let offset = if offset == std::ptr::null() {
        0
    } else {
        *offset
    };
    gl().vertex_attrib_pointer_f32(index, size as i32, data_type, normalized, stride, offset);
}

#[allow(non_snake_case)]
pub unsafe fn GenBuffers(_: types::GLsizei, buffer: &mut types::GLuint) {
    if let Ok(buf) = gl().create_buffer() {
        *buffer = buf.0.get();
    }
}

#[allow(non_snake_case)]
pub unsafe fn EnableVertexAttribArray(location: types::GLuint) {
    gl().enable_vertex_attrib_array(location);
}

#[allow(non_snake_case)]
pub unsafe fn BufferData(
    target: types::GLenum,
    size: types::GLsizeiptr,
    data: *const f64,
    usage: types::GLenum,
) {
    let data: *const u8 = std::mem::transmute(data);
    let data = std::slice::from_raw_parts(data, size);
    gl().buffer_data_u8_slice(target, data, usage);
}

#[allow(non_snake_case)]
pub unsafe fn CreateShader(shader_type: types::GLenum) -> types::GLuint {
    if let Ok(shader) = gl().create_shader(shader_type) {
        return shader.0.get();
    }
    return 0;
}

#[allow(non_snake_case)]
pub unsafe fn ShaderSource(
    shader: types::GLuint,
    _count: types::GLsizei,
    source: *const *const types::GLchar,
    _length: *const types::GLint,
) {
    let shader = NativeShader(NonZeroU32::new_unchecked(shader));
    let source = char_ptr_to_str(*source);
    gl().shader_source(shader, source);
}

#[allow(non_snake_case)]
pub unsafe fn CompileShader(shader: types::GLuint) {
    let shader = NativeShader(NonZeroU32::new_unchecked(shader));
    gl().compile_shader(shader);
}

#[allow(non_snake_case)]
pub unsafe fn GetCompleStatus(shader: types::GLuint, params: *mut types::GLint) {
    let shader = NativeShader(NonZeroU32::new_unchecked(shader));
    let status = gl().get_shader_compile_status(shader);
    *params = status as types::GLint;
}

#[allow(non_snake_case)]
pub unsafe fn GetShaderInfoLog(shader: types::GLuint) -> String {
    let shader = NativeShader(NonZeroU32::new_unchecked(shader));
    let log = gl().get_shader_info_log(shader);
    log
}

#[allow(non_snake_case)]
pub unsafe fn AttachShader(program: types::GLuint, shader: types::GLuint) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    let shader = NativeShader(NonZeroU32::new_unchecked(shader));
    gl().attach_shader(program, shader);
}

#[allow(non_snake_case)]
pub unsafe fn DeleteShader(shader: types::GLuint) {
    let shader = NativeShader(NonZeroU32::new_unchecked(shader));
    gl().delete_shader(shader);
}

#[allow(non_snake_case)]
pub unsafe fn GetAttribLocation(
    program: types::GLuint,
    name: *const types::GLchar,
) -> types::GLuint {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    gl().get_attrib_location(program, char_ptr_to_str(name))
        .unwrap_or(0)
}

#[allow(non_snake_case)]
pub unsafe fn Enable(parameter: types::GLenum) {
    gl().enable(parameter)
}

#[allow(non_snake_case)]
pub unsafe fn Disable(parameter: types::GLenum) {
    gl().disable(parameter)
}

#[allow(non_snake_case)]
pub unsafe fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) {
    gl().draw_arrays(mode, first, count)
}

#[allow(non_snake_case)]
pub unsafe fn BindFragDataLocation(
    program: types::GLuint,
    color: types::GLuint,
    name: *const types::GLchar,
) {
    let program = NativeProgram(NonZeroU32::new_unchecked(program));
    gl().bind_frag_data_location(program, color, char_ptr_to_str(name))
}

#[allow(non_snake_case)]
pub unsafe fn Viewport(
    x: types::GLint,
    y: types::GLint,
    width: types::GLsizei,
    height: types::GLsizei,
) {
    gl().viewport(x, y, width, height)
}

#[allow(non_snake_case)]
pub unsafe fn ClearColor(
    red: types::GLfloat,
    green: types::GLfloat,
    blue: types::GLfloat,
    alpha: types::GLfloat,
) {
    gl().clear_color(red, green, blue, alpha);
}

#[allow(non_snake_case)]
pub unsafe fn Clear(mask: types::GLbitfield) {
    gl().clear(mask);
}

#[allow(non_snake_case)]
pub unsafe fn ClearStencil(stencil: types::GLint) {
    gl().clear_stencil(stencil);
}

#[allow(non_snake_case)]
pub unsafe fn Scissor(
    x: types::GLint,
    y: types::GLint,
    width: types::GLsizei,
    height: types::GLsizei,
) {
    gl().scissor(x, y, width, height);
}

#[allow(non_snake_case)]
pub unsafe fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) {
    gl().stencil_func(func, ref_, mask);
}

#[allow(non_snake_case)]
pub unsafe fn StencilMask(mask: types::GLuint) {
    gl().stencil_mask(mask);
}

#[allow(non_snake_case)]
pub unsafe fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) {
    gl().stencil_op(fail, zfail, zpass);
}

#[allow(non_snake_case)]
pub unsafe fn BlendColor(
    red: types::GLfloat,
    green: types::GLfloat,
    blue: types::GLfloat,
    alpha: types::GLfloat,
) {
    gl().blend_color(red, green, blue, alpha);
}

#[allow(non_snake_case)]
pub unsafe fn BlendEquationSeparate(modeRGB: types::GLenum, modeAlpha: types::GLenum) {
    gl().blend_equation_separate(modeRGB, modeAlpha);
}

#[allow(non_snake_case)]
pub unsafe fn BlendFuncSeparate(
    sfactorRGB: types::GLenum,
    dfactorRGB: types::GLenum,
    sfactorAlpha: types::GLenum,
    dfactorAlpha: types::GLenum,
) {
    gl().blend_func_separate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
}

#[inline]
unsafe fn char_ptr_to_str<'a>(ptr: *const types::GLchar) -> &'a str {
    CStr::from_ptr(ptr).to_str().unwrap_or("")
}

#[allow(non_snake_case)]
pub mod Enable {
    pub fn is_loaded() -> bool {
        true
    }
}
