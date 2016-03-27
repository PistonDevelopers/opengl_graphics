/*
Use same binding of draw state as Gfx.
Source: https://github.com/gfx-rs/gfx_device_gl/blob/master/src/state.rs
*/

use gl;
// use graphics::draw_state::*;

/*
pub fn bind_state(old_state: &DrawState, new_state: &DrawState) {
    if old_state.primitive != new_state.primitive {
        bind_primitive(new_state.primitive);
    }
    if old_state.multi_sample != new_state.multi_sample {
        bind_multi_sample(new_state.multi_sample);
    }
    if old_state.scissor != new_state.scissor {
        bind_scissor(new_state.scissor);
    }
    if old_state.depth != new_state.depth
    || old_state.stencil != new_state.stencil
    || old_state.primitive.get_cull_face() !=
        new_state.primitive.get_cull_face() {
        bind_depth(new_state.depth);
        bind_stencil(new_state.stencil, new_state.primitive.get_cull_face());
    }
    if old_state.blend != new_state.blend {
        bind_blend(new_state.blend);
    }
    if old_state.color_mask != new_state.color_mask {
        bind_color_mask(new_state.color_mask);
    }
}

pub fn bind_primitive(p: Primitive) {
    unsafe { gl::FrontFace(match p.front_face {
        FrontFace::Clockwise => gl::CW,
        FrontFace::CounterClockwise => gl::CCW,
    }) };

    let (gl_draw, gl_offset) = match p.method {
        RasterMethod::Point => (gl::POINT, gl::POLYGON_OFFSET_POINT),
        RasterMethod::Line(width) => {
            unsafe { gl::LineWidth(width) };
            (gl::LINE, gl::POLYGON_OFFSET_LINE)
        },
        RasterMethod::Fill(cull) => {
            match cull {
                CullFace::Nothing => unsafe { gl::Disable(gl::CULL_FACE) },
                CullFace::Front => { unsafe {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT);
                }},
                CullFace::Back => { unsafe {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::BACK);
                }},
            }
            (gl::FILL, gl::POLYGON_OFFSET_FILL)
        },
    };

    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl_draw) };

    match p.offset {
        Some(Offset(factor, units)) => unsafe {
            gl::Enable(gl_offset);
            gl::PolygonOffset(factor, units as gl::types::GLfloat);
        },
        None => unsafe {
            gl::Disable(gl_offset)
        },
    }
}

pub fn bind_multi_sample(ms: Option<MultiSample>) {
    match ms {
        Some(_) => unsafe { gl::Enable(gl::MULTISAMPLE) },
        None => unsafe { gl::Disable(gl::MULTISAMPLE) },
    }
}

pub fn bind_scissor(rect: Option<Rect>) {
    match rect {
        Some(r) => { unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(
                r.x as gl::types::GLint,
                r.y as gl::types::GLint,
                r.w as gl::types::GLint,
                r.h as gl::types::GLint
            );
        }},
        None => unsafe { gl::Disable(gl::SCISSOR_TEST) },
    }
}

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

pub fn bind_depth(depth: Option<Depth>) {
    match depth {
        Some(d) => { unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(map_comparison(d.fun));
            gl::DepthMask(if d.write {gl::TRUE} else {gl::FALSE});
        }},
        None => unsafe { gl::Disable(gl::DEPTH_TEST) },
    }
}

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

pub fn bind_stencil(stencil: Option<Stencil>, cull: CullFace) {
    fn bind_side(face: gl::types::GLenum, side: StencilSide) { unsafe {
        gl::StencilFuncSeparate(face, map_comparison(side.fun),
            side.value as gl::types::GLint, side.mask_read as gl::types::GLuint);
        gl::StencilMaskSeparate(face, side.mask_write as gl::types::GLuint);
        gl::StencilOpSeparate(face, map_operation(side.op_fail),
            map_operation(side.op_depth_fail), map_operation(side.op_pass));
    }}
    match stencil {
        Some(s) => {
            unsafe { gl::Enable(gl::STENCIL_TEST) };
            if cull != CullFace::Front {
                bind_side(gl::FRONT, s.front);
            }
            if cull != CullFace::Back {
                bind_side(gl::BACK, s.back);
            }
        }
        None => unsafe { gl::Disable(gl::STENCIL_TEST) },
    }
}

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

pub fn bind_color_mask(mask: ColorMask) {
    unsafe { gl::ColorMask(
        if (mask & RED  ).is_empty() {gl::FALSE} else {gl::TRUE},
        if (mask & GREEN).is_empty() {gl::FALSE} else {gl::TRUE},
        if (mask & BLUE ).is_empty() {gl::FALSE} else {gl::TRUE},
        if (mask & ALPHA).is_empty() {gl::FALSE} else {gl::TRUE}
    )};
}
*/
