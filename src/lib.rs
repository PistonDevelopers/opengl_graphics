#![crate_id = "opengl_graphics"]
#![deny(missing_doc)]

//! An OpenGL back-end for Rust-Graphics

extern crate image;
extern crate gl;
extern crate libc;
extern crate graphics;

pub use gl_back_end::Gl;
pub use texture::Texture;

pub mod shader_utils;

mod gl_back_end;
mod texture;

