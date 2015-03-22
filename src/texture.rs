use gl;
use gl::types::GLuint;
use libc::c_void;
use texture_lib::{ TextureWithDevice, ImageSize, TexResult, TexError };

/// Wraps OpenGL texture data.
/// The texture gets deleted when running out of scope.
///
/// In order to create a texture the function `GenTextures` must be loaded.
/// This is done automatically by the window back-ends in Piston.
pub struct GlTexture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl GlTexture {
    /// Creates a new texture.
    #[inline(always)]
    pub fn new(id: GLuint, width: u32, height: u32) -> Self {
        GlTexture {
            id: id,
            width: width,
            height: height,
        }
    }

    /// Gets the OpenGL id of the texture.
    #[inline(always)]
    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

impl TextureWithDevice for GlTexture {
    type Device = ();

    fn from_memory(_: &mut <GlTexture as TextureWithDevice>::Device,
                   memory: &[u8], width: usize, channels: usize) -> TexResult<Self> {
        let tex = match channels {
            1 => {
                let mut tex = vec![];
                for &alpha in memory {
                    tex.extend(vec![255; 3]);
                    tex.push(alpha);
                }
                tex
            }
            4 => memory.to_vec(),
            n => return Err(TexError::Channels(n))
        };
        let height = memory.len() / width / channels;
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                tex.as_ptr() as *const c_void
            );
        }

        Ok(GlTexture::new(id, width as u32, height as u32))
    }

    fn update_from_memory(&mut self, _: &mut <GlTexture as TextureWithDevice>::Device,
                          memory: &[u8], width: usize, channels: usize) -> TexResult<()> {
        let tex = match channels {
            1 => {
                let mut tex = vec![];
                for &alpha in memory {
                    tex.extend(vec![255; 3]);
                    tex.push(alpha);
                }
                tex
            }
            4 => memory.to_vec(),
            n => return Err(TexError::Channels(n))
        };
        let height = memory.len() / width / channels;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                tex.as_ptr() as *const c_void
            );
        }
        Ok(())
    }
}

impl Drop for GlTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl ImageSize for GlTexture {
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
