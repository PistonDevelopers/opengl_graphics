use gl;
use gl::types::GLuint;
use image::{self, DynamicImage, RgbaImage};

use std::path::Path;

use {ops, ImageSize, CreateTexture, UpdateTexture, TextureOp, TextureSettings, Format, Filter, Wrap};

trait GlSettings {
    fn get_gl_mag(&self) -> gl::types::GLenum;
    fn get_gl_min(&self) -> gl::types::GLenum;
    fn get_gl_mipmap(&self) -> gl::types::GLenum;
    fn get_gl_wrap_u(&self) -> gl::types::GLenum;
    fn get_gl_wrap_v(&self) -> gl::types::GLenum;
}

impl GlSettings for TextureSettings {
    fn get_gl_mag(&self) -> gl::types::GLenum {
        match self.get_mag() {
            Filter::Linear => gl::LINEAR,
            Filter::Nearest => gl::NEAREST,
        }
    }

    fn get_gl_min(&self) -> gl::types::GLenum {
        match self.get_min() {
            Filter::Linear => {
                if self.get_generate_mipmap() {
                    match self.get_mipmap() {
                        Filter::Linear => gl::LINEAR_MIPMAP_LINEAR,
                        Filter::Nearest => gl::LINEAR_MIPMAP_NEAREST,
                    }
                } else {
                    gl::LINEAR
                }
            }
            Filter::Nearest => {
                if self.get_generate_mipmap() {
                    match self.get_mipmap() {
                        Filter::Linear => gl::NEAREST_MIPMAP_LINEAR,
                        Filter::Nearest => gl::NEAREST_MIPMAP_NEAREST,
                    }
                } else {
                    gl::NEAREST
                }
            }
        }
    }

    fn get_gl_mipmap(&self) -> gl::types::GLenum {
        match self.get_mipmap() {
            Filter::Linear => gl::LINEAR,
            Filter::Nearest => gl::NEAREST,
        }
    }

    fn get_gl_wrap_u(&self) -> gl::types::GLenum {
        match self.get_wrap_u() {
            Wrap::Repeat => gl::REPEAT,
            Wrap::MirroredRepeat => gl::MIRRORED_REPEAT,
            Wrap::ClampToEdge => gl::CLAMP_TO_EDGE,
            Wrap::ClampToBorder => gl::CLAMP_TO_BORDER,
        }
    }

    fn get_gl_wrap_v(&self) -> gl::types::GLenum {
        match self.get_wrap_v() {
            Wrap::Repeat => gl::REPEAT,
            Wrap::MirroredRepeat => gl::MIRRORED_REPEAT,
            Wrap::ClampToEdge => gl::CLAMP_TO_EDGE,
            Wrap::ClampToBorder => gl::CLAMP_TO_BORDER,
        }
    }

}

/// Wraps OpenGL texture data.
/// The texture gets deleted when running out of scope.
///
/// In order to create a texture the function `GenTextures` must be loaded.
/// This is done automatically by the window back-ends in Piston.
pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    /// Creates a new texture.
    #[inline(always)]
    pub fn new(id: GLuint, width: u32, height: u32) -> Self {
        Texture {
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

    /// Returns empty texture.
    pub fn empty(settings: &TextureSettings) -> Result<Self, String> {
        CreateTexture::create(&mut (),
                              Format::Rgba8,
                              &[0u8; 4],
                              [1, 1],
                              settings)
    }

    /// Loads image from memory, the format is 8-bit greyscale.
    pub fn from_memory_alpha(buf: &[u8],
                             width: u32,
                             height: u32,
                             settings: &TextureSettings)
                             -> Result<Self, String> {
        let size = [width, height];
        let buffer = ops::alpha_to_rgba8(buf, size);
        CreateTexture::create(&mut (), Format::Rgba8, &buffer, size, settings)
    }

    /// Loads image by relative file name to the asset root.
    pub fn from_path<P>(path: P, settings: &TextureSettings) -> Result<Self, String>
        where P: AsRef<Path>
    {
        let path = path.as_ref();

        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => {
                return Err(format!("Could not load '{:?}': {:?}", path.file_name().unwrap(), e))
            }
        };

        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba(),
        };

        Ok(Texture::from_image(&img, settings))
    }

    /// Creates a texture from image.
    pub fn from_image(img: &RgbaImage, settings: &TextureSettings) -> Self {
        let (width, height) = img.dimensions();
        CreateTexture::create(&mut (), Format::Rgba8, img, [width, height], settings).unwrap()
    }

    /// Updates image with a new one.
    pub fn update(&mut self, img: &RgbaImage) {
        let (width, height) = img.dimensions();

        UpdateTexture::update(self, &mut (), Format::Rgba8, img, [0, 0], [width, height]).unwrap();
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            let ids = [self.id];
            gl::DeleteTextures(1, ids.as_ptr());
            drop(ids);
        }
    }
}

impl ImageSize for Texture {
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl TextureOp<()> for Texture {
    type Error = String;
}

impl CreateTexture<()> for Texture {
    fn create<S: Into<[u32; 2]>>(_factory: &mut (),
                                 _format: Format,
                                 memory: &[u8],
                                 size: S,
                                 settings: &TextureSettings)
                                 -> Result<Self, Self::Error> {
        let size = size.into();
        let mut id: GLuint = 0;
        let internal_format = if settings.get_convert_gamma() {
            gl::RGBA
        } else {
            gl::SRGB_ALPHA
        };
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D,
                              gl::TEXTURE_MIN_FILTER,
                              settings.get_gl_min() as i32);
            gl::TexParameteri(gl::TEXTURE_2D,
                              gl::TEXTURE_MAG_FILTER,
                              settings.get_gl_mag() as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, settings.get_gl_wrap_u() as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, settings.get_gl_wrap_v() as i32);
            if settings.get_wrap_u() == Wrap::ClampToBorder ||
                settings.get_wrap_v() == Wrap::ClampToBorder {
                gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, settings.get_border_color().as_ptr());
            }
            if settings.get_generate_mipmap() {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           internal_format as i32,
                           size[0] as i32,
                           size[1] as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           memory.as_ptr() as *const _);
        }

        Ok(Texture::new(id, size[0], size[1]))
    }
}

impl UpdateTexture<()> for Texture {
    fn update<O: Into<[u32; 2]>, S: Into<[u32; 2]>>(&mut self,
                                                    _factory: &mut (),
                                                    _format: Format,
                                                    memory: &[u8],
                                                    offset: O,
                                                    size: S)
                                                    -> Result<(), Self::Error> {
        let offset = offset.into();
        let size = size.into();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexSubImage2D(gl::TEXTURE_2D,
                              0,
                              offset[0] as i32,
                              offset[1] as i32,
                              size[0] as i32,
                              size[1] as i32,
                              gl::RGBA,
                              gl::UNSIGNED_BYTE,
                              memory.as_ptr() as *const _);
        }

        Ok(())
    }
}
