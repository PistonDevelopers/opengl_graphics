#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! An OpenGL back-end for Rust-Graphics

extern crate shader_version;
extern crate shaders_graphics2d as shaders;
extern crate image;
extern crate gl;
extern crate graphics;
extern crate texture as texture_lib;

pub use shader_version::{OpenGL, Shaders};
pub use shader_version::glsl::{GLSL};
pub use back_end::{Colored, Textured, GlGraphics};
pub use texture::Texture;
pub use texture_lib::*;

pub mod shader_utils;
pub mod error;
pub mod shader_uniforms;

/// Glyph cache implementation for OpenGL backend.
pub type GlyphCache<'a> = graphics::glyph_cache::rusttype::GlyphCache<'a, (), Texture>;

mod back_end;
mod texture;
mod draw_state;
