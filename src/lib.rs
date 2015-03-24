#![crate_name = "opengl_graphics"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![feature(std_misc, old_path)]

//! An OpenGL back-end for Rust-Graphics

extern crate shader_version;
extern crate gl;
extern crate libc;
extern crate graphics;
extern crate freetype;
extern crate "texture" as texture_lib;

pub use shader_version::OpenGL;
pub use gl_back_end::GlGraphics;
pub use gl_back_end::GlGraphics as Gl;
pub use texture::GlTexture;
pub use texture_lib::Texture;

pub mod shader_utils;
pub mod glyph_cache;
pub mod error;

mod gl_back_end;
mod texture;
mod shaders;
