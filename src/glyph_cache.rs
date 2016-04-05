//! Glyph caching

use { freetype, graphics, Texture, TextureSettings };
use std::collections::HashMap;
use graphics::types::Scalar;

extern crate fnv;
use self::fnv::FnvHasher;
use std::hash::BuildHasherDefault;

use std::path::Path;
use error::Error;

pub use graphics::types::FontSize;

/// The type alias for font characters.
pub type Character<'a> = graphics::character::Character<'a, Texture>;

/// A struct used for caching rendered font.
pub struct GlyphCache<'a> {
    /// The font face.
    pub face: freetype::Face<'a>,
    // Maps from fontsize and character to offset, size and texture.
    data: HashMap<(FontSize, char),
                  ([Scalar; 2], [Scalar; 2], Texture),
                  BuildHasherDefault<FnvHasher>>,
}

impl<'a> GlyphCache<'a> {
    /// Constructor for a GlyphCache.
    pub fn new(font: &Path) -> Result<GlyphCache<'static>, Error> {
        let freetype = match freetype::Library::init() {
            Ok(freetype) => freetype,
            Err(why) => return Err(Error::FreetypeError(why)),
        };
        let face = match freetype.new_face(font, 0) {
            Ok(face) => face,
            Err(why) => return Err(Error::FreetypeError(why)),
        };
        Ok(GlyphCache {
            face: face,
            data: HashMap::with_hasher(BuildHasherDefault::<FnvHasher>::default()),
        })
    }

    /// Creates a GlyphCache for a font stored in memory.
    pub fn from_bytes(font: &'a [u8]) -> Result<GlyphCache<'a>, Error> {
        let freetype = match freetype::Library::init() {
            Ok(freetype) => freetype,
            Err(why) => return Err(Error::FreetypeError(why))
        };
        let face = match freetype.new_memory_face(font, 0) {
            Ok(face) => face,
            Err(why) => return Err(Error::FreetypeError(why))
        };
        Ok(GlyphCache {
            face: face,
            data: HashMap::with_hasher(BuildHasherDefault::<FnvHasher>::default())
        })
    }

    /// Get a `Character` from cache, or load it if not there.
    fn get(&mut self, size: FontSize,  ch: char)
    -> &([Scalar; 2], [Scalar; 2], Texture) {
        // Create a `Character` from a given `FontSize` and `char`.
        fn create_character(face: &freetype::Face, size: FontSize, ch: char)
        -> ([Scalar; 2], [Scalar; 2], Texture) {
            face.set_pixel_sizes(0, size).unwrap();
            face.load_char(ch as usize, freetype::face::DEFAULT).unwrap();
            let glyph = face.glyph().get_glyph().unwrap();
            let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::RenderMode::Normal, None)
                .unwrap();
            let bitmap = bitmap_glyph.bitmap();
            let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                     bitmap.width() as u32,
                                                     bitmap.rows() as u32,
                                                     &TextureSettings::new()).unwrap();
            (
                [bitmap_glyph.left() as f64, bitmap_glyph.top() as f64],
                [(glyph.advance_x() >> 16) as f64, (glyph.advance_y() >> 16) as f64],
                texture,
            )
        }

        let face = &self.face;// necessary to borrow-check
        self.data.entry((size, ch))
                 .or_insert_with(|| create_character(face, size, ch) )
    }

    /// Load all characters in the `chars` iterator for `size`
    pub fn preload_chars<I>(
        &mut self,
        size: FontSize,
        chars: I
    )
        where
            I: Iterator<Item = char>
    {
        for ch in chars {
            self.get(size, ch);
        }
    }

    /// Load all the printable ASCII characters for `size`. Includes space.
    pub fn preload_printable_ascii(&mut self, size: FontSize) {
        // [0x20, 0x7F) contains all printable ASCII characters ([' ', '~'])
        self.preload_chars(size, (0x20u8 .. 0x7F).map(|ch| ch as char));
    }

    /// Return `ch` for `size` if it's already cached. Don't load.
    /// See the `preload_*` functions.
    pub fn opt_character(&self, size: FontSize, ch: char) -> Option<Character> {
        self.data.get(&(size, ch)).map(|&(offset, size, ref texture)| {
            Character {
                offset: offset,
                size: size,
                texture: texture
            }
        })
    }
}

impl<'b> graphics::character::CharacterCache for GlyphCache<'b> {
    type Texture = Texture;

    fn character<'a>(&'a mut self, size: FontSize, ch: char) -> Character<'a> {
        let &(offset, size, ref texture) = self.get(size, ch);
        return Character {
            offset: offset,
            size: size,
            texture: texture
        }
    }
}
