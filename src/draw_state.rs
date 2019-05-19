use gl;
use graphics::draw_state::*;
use viewport::Viewport;

pub fn bind_state(old_state: &DrawState, new_state: &DrawState, viewport: &Option<Viewport>) {
    if old_state.scissor != new_state.scissor {
        bind_scissor(new_state.scissor, viewport);
    }
    if old_state.stencil != new_state.stencil {
        bind_stencil(new_state.stencil);
    }
    if old_state.blend != new_state.blend {
        bind_blend(new_state.blend);
    }
}

pub fn bind_scissor(rect: Option<[u32; 4]>, viewport: &Option<Viewport>) {
    match rect {
        Some(r) => {
            // https://www.khronos.org/opengl/wiki/Scissor_Test indicates that 
            // gl::Scissor takes x,y defined as lower left,
            // but piston passes rect with x,y defined as upper left.
            // To fix this we need to know height of the viewport
            // so that we can transform y as top measured from top (yt)
            // into y as bottom measured from bottom (yb)
            // using yb = viewport_height - (yt + rect_height)
            let yb = if let Some(vp) = viewport {
                vp.rect[3] - (r[1] + r[3]) as i32
            } else {
                r[1] as i32
            };
            unsafe {
                gl::Enable(gl::SCISSOR_TEST);
                gl::Scissor(r[0] as gl::types::GLint,
                            yb as gl::types::GLint,
                            r[2] as gl::types::GLint,
                            r[3] as gl::types::GLint);
            }
        },
        None => unsafe { gl::Disable(gl::SCISSOR_TEST) },
    }
}

pub fn bind_stencil(stencil: Option<Stencil>) {
    unsafe {
        match stencil {
            Some(s) => {
                gl::Enable(gl::STENCIL_TEST);
                match s {
                    Stencil::Clip(val) => {
                        gl::StencilFunc(gl::NEVER, val as gl::types::GLint, 255);
                        gl::StencilMask(255);
                        gl::StencilOp(gl::REPLACE, gl::KEEP, gl::KEEP);
                    }
                    Stencil::Inside(val) => {
                        gl::StencilFunc(gl::EQUAL, val as gl::types::GLint, 255);
                        gl::StencilMask(255);
                        gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
                    }
                    Stencil::Outside(val) => {
                        gl::StencilFunc(gl::NOTEQUAL, val as gl::types::GLint, 255);
                        gl::StencilMask(255);
                        gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
                    }
                }
            }
            None => gl::Disable(gl::STENCIL_TEST),
        }
    }
}

/*
fn map_equation(eq: Equation) -> gl::types::GLenum {
    match eq {
        Equation::Add    => gl::FUNC_ADD,
        Equation::Sub    => gl::FUNC_SUBTRACT,
        Equation::RevSub => gl::FUNC_REVERSE_SUBTRACT,
        Equation::Min    => gl::MIN,
        Equation::Max    => gl::MAX,
    }
}

fn map_factor(factor: Factor) -> gl::types::GLenum {
    match factor {
        Factor::Zero                              => gl::ZERO,
        Factor::One                               => gl::ONE,
        Factor::ZeroPlus(BlendValue::SourceColor) => gl::SRC_COLOR,
        Factor::OneMinus(BlendValue::SourceColor) => gl::ONE_MINUS_SRC_COLOR,
        Factor::ZeroPlus(BlendValue::SourceAlpha) => gl::SRC_ALPHA,
        Factor::OneMinus(BlendValue::SourceAlpha) => gl::ONE_MINUS_SRC_ALPHA,
        Factor::ZeroPlus(BlendValue::DestColor)   => gl::DST_COLOR,
        Factor::OneMinus(BlendValue::DestColor)   => gl::ONE_MINUS_DST_COLOR,
        Factor::ZeroPlus(BlendValue::DestAlpha)   => gl::DST_ALPHA,
        Factor::OneMinus(BlendValue::DestAlpha)   => gl::ONE_MINUS_DST_ALPHA,
        Factor::ZeroPlus(BlendValue::ConstColor)  => gl::CONSTANT_COLOR,
        Factor::OneMinus(BlendValue::ConstColor)  => gl::ONE_MINUS_CONSTANT_COLOR,
        Factor::ZeroPlus(BlendValue::ConstAlpha)  => gl::CONSTANT_ALPHA,
        Factor::OneMinus(BlendValue::ConstAlpha)  => gl::ONE_MINUS_CONSTANT_ALPHA,
        Factor::SourceAlphaSaturated => gl::SRC_ALPHA_SATURATE,
    }
}
*/

pub fn bind_blend(blend: Option<Blend>) {
    unsafe {
        match blend {
            Some(b) => {
                gl::Enable(gl::BLEND);
                gl::BlendColor(1.0, 1.0, 1.0, 1.0);
                match b {
                    Blend::Alpha => {
                        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
                        gl::BlendFuncSeparate(gl::SRC_ALPHA,
                                              gl::ONE_MINUS_SRC_ALPHA,
                                              gl::ONE,
                                              gl::ONE);
                    }
                    Blend::Add => {
                        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
                        gl::BlendFuncSeparate(gl::ONE, gl::ONE, gl::ONE, gl::ONE);
                    }
                    Blend::Multiply => {
                        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
                        gl::BlendFuncSeparate(gl::DST_COLOR, gl::ZERO, gl::DST_ALPHA, gl::ZERO);
                    }
                    Blend::Invert => {
                        gl::BlendEquationSeparate(gl::FUNC_SUBTRACT, gl::FUNC_ADD);
                        gl::BlendFuncSeparate(gl::CONSTANT_COLOR, gl::SRC_COLOR, gl::ZERO, gl::ONE);
                    }
                }
            }
            None => gl::Disable(gl::BLEND),
        }
    }
}
