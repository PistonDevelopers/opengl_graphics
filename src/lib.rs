#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! An OpenGL back-end for Rust-Graphics

extern crate shader_version;
extern crate shaders_graphics2d as shaders;
extern crate image;
extern crate gl;
extern crate graphics;
extern crate rusttype;
extern crate texture as texture_lib;

pub use shader_version::OpenGL;
pub use back_end::GlGraphics;
pub use texture::Texture;
pub use texture_lib::*;

pub mod shader_utils;
pub mod glyph_cache;
pub mod error;

mod back_end;
mod texture;
mod draw_state;
