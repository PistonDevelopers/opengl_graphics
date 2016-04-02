/*
Use same binding of draw state as Gfx.
Source: https://github.com/gfx-rs/gfx_device_gl/blob/master/src/state.rs
*/

use gl;
use graphics::draw_state::*;

pub fn bind_state(old_state: &DrawState, new_state: &DrawState) {
    if old_state.scissor != new_state.scissor {
        bind_scissor(new_state.scissor);
    }
    if old_state.stencil != new_state.stencil {
        bind_stencil(new_state.stencil);
    }
    if old_state.blend != new_state.blend {
        // bind_blend(new_state.blend);
    }
}

pub fn bind_scissor(rect: Option<[u32; 4]>) {
    match rect {
        Some(r) => { unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(
                r[0] as gl::types::GLint,
                r[1] as gl::types::GLint,
                r[2] as gl::types::GLint,
                r[3] as gl::types::GLint
            );
        }},
        None => unsafe { gl::Disable(gl::SCISSOR_TEST) },
    }
}

/*
fn map_comparison(cmp: Comparison) -> gl::types::GLenum {
    match cmp {
        Comparison::Never        => gl::NEVER,
        Comparison::Less         => gl::LESS,
        Comparison::LessEqual    => gl::LEQUAL,
        Comparison::Equal        => gl::EQUAL,
        Comparison::GreaterEqual => gl::GEQUAL,
        Comparison::Greater      => gl::GREATER,
        Comparison::NotEqual     => gl::NOTEQUAL,
        Comparison::Always       => gl::ALWAYS,
    }
}
*/

/*
fn map_operation(op: StencilOp) -> gl::types::GLenum {
    match op {
        StencilOp::Keep          => gl::KEEP,
        StencilOp::Zero          => gl::ZERO,
        StencilOp::Replace       => gl::REPLACE,
        StencilOp::IncrementClamp=> gl::INCR,
        StencilOp::IncrementWrap => gl::INCR_WRAP,
        StencilOp::DecrementClamp=> gl::DECR,
        StencilOp::DecrementWrap => gl::DECR_WRAP,
        StencilOp::Invert        => gl::INVERT,
    }
}
*/

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

pub fn bind_blend(blend: Option<Blend>) {
    match blend {
        Some(b) => { unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendEquationSeparate(
                map_equation(b.color.equation),
                map_equation(b.alpha.equation)
            );
            gl::BlendFuncSeparate(
                map_factor(b.color.source),
                map_factor(b.color.destination),
                map_factor(b.alpha.source),
                map_factor(b.alpha.destination)
            );
            let c = b.value;
            gl::BlendColor(c[0], c[1], c[2], c[3]);
        }},
        None => unsafe { gl::Disable(gl::BLEND) },
    }
}
*/
