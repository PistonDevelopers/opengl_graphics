#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! An OpenGL back-end for Rust-Graphics

extern crate shader_version;
extern crate image;
extern crate gl;
extern crate libc;
extern crate graphics;
extern crate freetype;
extern crate viewport;

pub use shader_version::OpenGL;
pub use gl_back_end::GlGraphics;
pub use gl_back_end::GlGraphics as Gl;
pub use texture::Texture;
pub use viewport::Viewport;

pub mod shader_utils;
pub mod glyph_cache;
pub mod error;

mod gl_back_end;
mod texture;
mod shaders;
