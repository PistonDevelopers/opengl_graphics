#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! An OpenGL back-end for Rust-Graphics

#[cfg(not(feature = "glow"))]
extern crate gl;
#[cfg(feature = "glow")]
extern crate glow_wrap as gl;
extern crate graphics;
extern crate image;
extern crate shader_version;
extern crate shaders_graphics2d as shaders;
extern crate texture as texture_lib;
extern crate viewport;

pub use crate::back_end::{Colored, GlGraphics, Textured, TexturedColor};
pub use crate::texture::Texture;
pub use shader_version::glsl::GLSL;
pub use shader_version::{OpenGL, Shaders};
pub use texture_lib::*;

pub mod error;
pub mod shader_uniforms;
pub mod shader_utils;

/// Glyph cache implementation for OpenGL backend.
pub type GlyphCache<'a> = graphics::glyph_cache::rusttype::GlyphCache<'a, (), Texture>;

mod back_end;
mod draw_state;
mod texture;

#[cfg(feature = "glow")]
pub use gl::set_context;
