//! Errors

/// An enum to represent various possible run-time errors that may occur.
#[deriving(Show, PartialEq, Eq)]
pub enum Error {
    /// An error happened with the FreeType library.
    FreetypeError(::freetype::error::Error),
}
