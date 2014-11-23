#![crate_name = "opengl_graphics"]
#![deny(missing_docs)]

//! An OpenGL back-end for Rust-Graphics

extern crate shader_version;
extern crate image;
extern crate gl;
extern crate libc;
extern crate graphics;
extern crate freetype;

pub use gl_back_end::Gl;
pub use texture::Texture;

pub mod shader_utils;
pub mod glyph_cache;
pub mod error;

mod gl_back_end;
mod texture;
