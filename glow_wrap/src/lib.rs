use std::ffi::CStr;
use std::sync::Arc;
use std::sync::OnceLock;
#[cfg(target_arch = "wasm32")]
use std::{borrow::BorrowMut, cell::RefCell};

use glow::HasContext as _;

mod key;
use key::ConvertKey;

struct Ref<T>(T);
unsafe impl<T> Send for Ref<T> {}
unsafe impl<T> Sync for Ref<T> {}

#[cfg(not(target_arch = "wasm32"))]
static CONTEXT: OnceLock<Arc<glow::Context>> = OnceLock::new();
#[cfg(target_arch = "wasm32")]
static CONTEXT: OnceLock<Ref<Arc<glow::Context>>> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
slotmap::new_key_type! { pub struct WebGlUniformLocationKey; }
#[cfg(target_arch = "wasm32")]
unsafe impl Send for WebGlUniformLocationKey {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for WebGlUniformLocationKey {}

#[cfg(target_arch = "wasm32")]
static LOCATION: OnceLock<
    Ref<RefCell<slotmap::SlotMap<WebGlUniformLocationKey, web_sys::WebGlUniformLocation>>>,
> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
pub fn set_context(ctx: Arc<glow::Context>) {
    let _ = CONTEXT.set(ctx);
}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn gl() -> &'static glow::Context {
    CONTEXT.get().unwrap().as_ref()
}

#[cfg(target_arch = "wasm32")]
pub fn init(ctx: Arc<glow::Context>) {
    use slotmap::SlotMap;
    let _ = CONTEXT.set(Ref(ctx));
    let _ = LOCATION.set(Ref(RefCell::new(SlotMap::with_key())));
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn gl() -> &'static glow::Context {
    CONTEXT.get().unwrap().0.as_ref()
}

pub mod types {
    use std::usize;

    pub type GLfloat = f32;
    pub type GLenum = u32;
    pub type GLuint = u32;
    pub type GLsizei = i32;
    pub type GLboolean = u8;
    pub type GLint = i32;
    pub type GLchar = std::ffi::c_char;
    pub type GLsizeiptr = usize;
    pub type GLbitfield = u32;
}

pub use glow::{
    ARRAY_BUFFER, BLEND, CLAMP_TO_BORDER, CLAMP_TO_EDGE, COLOR_BUFFER_BIT, COMPILE_STATUS,
    CONSTANT_COLOR, CULL_FACE, DEPTH_BUFFER_BIT, DST_ALPHA, DST_COLOR, DYNAMIC_DRAW, EQUAL, FALSE,
    FLOAT, FRAGMENT_SHADER, FRAMEBUFFER_SRGB, FUNC_ADD, FUNC_SUBTRACT, INCR, KEEP, LINEAR,
    LINEAR_MIPMAP_LINEAR, LINEAR_MIPMAP_NEAREST, MIRRORED_REPEAT, NEAREST, NEAREST_MIPMAP_LINEAR,
    NEAREST_MIPMAP_NEAREST, NEVER, NOTEQUAL, ONE, ONE_MINUS_SRC_ALPHA, REPEAT, REPLACE, RGBA,
    SCISSOR_TEST, SRC_ALPHA, SRC_COLOR, SRGB_ALPHA, STENCIL_BUFFER_BIT, STENCIL_TEST, TEXTURE_2D,
    TEXTURE_BORDER_COLOR, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T,
    TRIANGLES, TRUE, UNSIGNED_BYTE, VERTEX_SHADER, ZERO,
};

#[allow(non_snake_case)]
pub unsafe fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) {
    let textures = std::slice::from_raw_parts(textures, n as usize);
    for texture in textures {
        gl().delete_texture(glow::Texture::from_key(*texture));
    }
}

#[allow(non_snake_case)]
pub unsafe fn GenTextures(_: types::GLsizei, textures: *mut types::GLuint) {
    if let Ok(id) = gl().create_texture() {
        *textures = glow::Texture::to_key(&id);
    }
}

#[allow(non_snake_case)]
pub unsafe fn BindTexture(target: u32, texture: types::GLuint) {
    gl().bind_texture(target, Some(glow::Texture::from_key(texture)));
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
    debug_assert!(format == glow::SRGB_ALPHA || format == glow::RGBA);
    let pixels = std::slice::from_raw_parts(pixels, (width * height * 4) as usize);
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
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: u32,
    ty: u32,
    pixels: *const u8,
) {
    debug_assert!(format == glow::SRGB_ALPHA || format == glow::RGBA);
    let pixels = std::slice::from_raw_parts(pixels, (width * height * 4) as usize);
    gl().tex_sub_image_2d(
        target,
        level,
        x_offset,
        y_offset,
        width,
        height,
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
    let location = gl().get_uniform_location(glow::Program::from_key(program), name);

    if let Some(location) = location {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(Ref(slot)) = LOCATION.get().borrow_mut().as_mut() {
                let key = slot.borrow_mut().insert(location);
                return WebGlUniformLocationKey::to_key(&key) as types::GLint;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            return glow::UniformLocation::to_key(&location) as types::GLint;
        }
    }

    return 0;
}

#[cfg(target_arch = "wasm32")]
unsafe fn get_uniform<F>(location: types::GLuint, f: F)
where
    F: FnOnce(&web_sys::WebGlUniformLocation),
{
    let key = WebGlUniformLocationKey::from_key(location);
    if let Some(Ref(slot)) = LOCATION.get().borrow_mut().as_mut() {
        if let Some(location) = slot.borrow_mut().get(key) {
            f(&location);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn get_uniform<F>(location: types::GLuint, f: F)
where
    F: FnOnce(&glow::UniformLocation),
{
    let location = glow::UniformLocation::from_key(location);
    f(&location);
}

#[allow(non_snake_case)]
pub unsafe fn CreateProgram() -> types::GLuint {
    match gl().create_program() {
        Ok(program) => return glow::Program::to_key(&program),
        Err(_) => {}
    };
    return 0;
}

#[allow(non_snake_case)]
pub unsafe fn LinkProgram(program: types::GLuint) {
    let program = glow::Program::from_key(program);
    gl().link_program(program);
}

#[allow(non_snake_case)]
pub unsafe fn UseProgram(program: types::GLuint) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
}

#[allow(non_snake_case)]
pub unsafe fn DeleteProgram(program: types::GLuint) {
    let program = glow::Program::from_key(program);
    gl().delete_program(program);
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform1f(program: types::GLuint, location: types::GLint, value: f32) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));

    get_uniform(location as u32, |location| {
        gl().uniform_1_f32(Some(&location), value);
    });
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform1i(program: types::GLuint, location: types::GLint, value: i32) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
    get_uniform(location as u32, |location| {
        gl().uniform_1_i32(Some(&location), value);
    });
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform2f(program: types::GLuint, location: types::GLint, x: f32, y: f32) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
    get_uniform(location as u32, |location| {
        gl().uniform_2_f32(Some(&location), x, y);
    });
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniform3f(
    program: types::GLuint,
    location: types::GLint,
    x: f32,
    y: f32,
    z: f32,
) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));

    get_uniform(location as u32, |location| {
        gl().uniform_3_f32(Some(&location), x, y, z);
    });
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
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
    get_uniform(location as u32, |location| {
        gl().uniform_4_f32(Some(&location), x, y, z, w);
    });
}

#[allow(non_snake_case)]
pub unsafe fn Uniform4f(location: types::GLint, x: f32, y: f32, z: f32, w: f32) {
    get_uniform(location as u32, |location| {
        gl().uniform_4_f32(Some(&location), x, y, z, w);
    });
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniformMatrix2fv(
    program: types::GLuint,
    location: types::GLint,
    count: types::GLint,
    transpose: types::GLboolean,
    value: *const f32,
) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
    let value = std::slice::from_raw_parts(value, count as usize * 4);

    get_uniform(location as u32, |location| {
        gl().uniform_matrix_2_f32_slice(Some(&location), transpose == TRUE, value);
    });
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniformMatrix3fv(
    program: types::GLuint,
    location: types::GLint,
    count: types::GLint,
    transpose: types::GLboolean,
    value: *const f32,
) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
    let value = std::slice::from_raw_parts(value, count as usize * 9);

    get_uniform(location as u32, |location| {
        gl().uniform_matrix_3_f32_slice(Some(&location), transpose == TRUE, value);
    });
}

#[allow(non_snake_case)]
pub unsafe fn ProgramUniformMatrix4fv(
    program: types::GLuint,
    location: types::GLint,
    count: types::GLint,
    transpose: types::GLboolean,
    value: *const f32,
) {
    let program = glow::Program::from_key(program);
    gl().use_program(Some(program));
    let value = std::slice::from_raw_parts(value, count as usize * 16);

    get_uniform(location as u32, |location| {
        gl().uniform_matrix_4_f32_slice(Some(&location), transpose == TRUE, value);
    });
}

#[allow(non_snake_case)]
pub unsafe fn DeleteBuffers(_: types::GLsizei, buffer: &types::GLuint) {
    let buffer = glow::Buffer::from_key(*buffer);
    gl().delete_buffer(buffer)
}

#[allow(non_snake_case)]
pub unsafe fn BindVertexArray(array: types::GLuint) {
    if array > 0 {
        let array = glow::VertexArray::from_key(array);
        gl().bind_vertex_array(Some(array));
    }
}

#[allow(non_snake_case)]
pub unsafe fn GenVertexArrays(_: types::GLsizei, arrays: *mut types::GLuint) {
    match gl().create_vertex_array() {
        Ok(array) => *arrays = glow::VertexArray::to_key(&array),
        Err(_) => {}
    };
}

#[allow(non_snake_case)]
pub unsafe fn DeleteVertexArrays(_: types::GLsizei, array: *const types::GLuint) {
    let array = glow::VertexArray::from_key(*array);
    gl().delete_vertex_array(array);
}

#[allow(non_snake_case)]
pub unsafe fn BindBuffer(target: types::GLenum, buffer: types::GLuint) {
    let buffer = glow::Buffer::from_key(buffer);
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
    gl().vertex_attrib_pointer_f32(
        index,
        size as i32,
        data_type,
        normalized == TRUE,
        stride,
        offset,
    );
}

#[allow(non_snake_case)]
pub unsafe fn GenBuffers(_: types::GLsizei, buffer: &mut types::GLuint) {
    if let Ok(buf) = gl().create_buffer() {
        *buffer = glow::Buffer::to_key(&buf);
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
        return glow::Shader::to_key(&shader);
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
    let shader = glow::Shader::from_key(shader);
    let source = char_ptr_to_str(*source);
    gl().shader_source(shader, source);
}

#[allow(non_snake_case)]
pub unsafe fn CompileShader(shader: types::GLuint) {
    let shader = glow::Shader::from_key(shader);
    gl().compile_shader(shader);
}

#[allow(non_snake_case)]
pub unsafe fn GetCompleStatus(shader: types::GLuint, params: *mut types::GLint) {
    let shader = glow::Shader::from_key(shader);
    let status = gl().get_shader_compile_status(shader);
    *params = status as types::GLint;
}

#[allow(non_snake_case)]
pub unsafe fn GetShaderInfoLog(shader: types::GLuint) -> String {
    let shader = glow::Shader::from_key(shader);
    let log = gl().get_shader_info_log(shader);
    log
}

#[allow(non_snake_case)]
pub unsafe fn AttachShader(program: types::GLuint, shader: types::GLuint) {
    let program = glow::Program::from_key(program);
    let shader = glow::Shader::from_key(shader);
    gl().attach_shader(program, shader);
}

#[allow(non_snake_case)]
pub unsafe fn DeleteShader(shader: types::GLuint) {
    let shader = glow::Shader::from_key(shader);
    gl().delete_shader(shader);
}

#[allow(non_snake_case)]
pub unsafe fn GetAttribLocation(
    program: types::GLuint,
    name: *const types::GLchar,
) -> types::GLint {
    let program = glow::Program::from_key(program);
    gl().get_attrib_location(program, char_ptr_to_str(name))
        .map(|l| l as i32)
        .unwrap_or(-1)
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
    let program = glow::Program::from_key(program);
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
