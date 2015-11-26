use gl;
use gl::types::GLuint;
use image::{ self, DynamicImage, GenericImage, RgbaImage };

use std::path::Path;

use { ops, ImageSize, Rgba8Texture, TextureSettings };

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

    /// Loads image from memory, the format is 8-bit greyscale.
    pub fn from_memory_alpha(buf: &[u8], width: u32, height: u32) -> Result<Self, String> {
        let size = [width, height];
        let buffer = ops::alpha_to_rgba8(buf, size);
        Rgba8Texture::create(&mut (), &buffer, size, &TextureSettings::new())
    }

    /// Loads image by relative file name to the asset root.
    pub fn from_path<P>(path: P) -> Result<Self, String> where P: AsRef<Path> {
		let path = path.as_ref();

        let img = match image::open(path) {
            Ok(img) => img,
            Err(e)  => return Err(format!("Could not load '{:?}': {:?}",
                path.file_name().unwrap(), e)),
        };

        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba()
        };

        Ok(Texture::from_image(&img))
    }

    /// Creates a texture from image.
    pub fn from_image(img: &RgbaImage) -> Self {
        let (width, height) = img.dimensions();
        Rgba8Texture::create(&mut (), img, [width, height], &TextureSettings::new()).unwrap()
    }

    /// Updates image with a new one.
    pub fn update(&mut self, img: &RgbaImage) {
        let (width, height) = img.dimensions();

        Rgba8Texture::update(self, &mut (), img, [width, height]).unwrap();
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl ImageSize for Texture {
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Rgba8Texture<()> for Texture {
    type Error = String;

    fn create<S: Into<[u32; 2]>>(
        _factory: &mut (),
        memory: &[u8],
        size: S,
        _settings: &TextureSettings
    ) -> Result<Self, Self::Error> {
        let size = size.into();
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
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                size[0] as i32,
                size[1] as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                memory.as_ptr() as *const _
            );
        }

        Ok(Texture::new(id, size[0], size[1]))
    }

    fn update<S: Into<[u32; 2]>>(
        &mut self,
        _factory: &mut (),
        memory: &[u8],
        size: S
    ) -> Result<(), Self::Error> {
        let size = size.into();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                size[0] as i32,
                size[1] as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                memory.as_ptr() as *const _
            );
        }

        Ok(())
    }
}
