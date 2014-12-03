
//! Glyph caching

use error::Error;
use freetype::ffi;
use freetype;
use freetype::error::Error::MissingFontField;
use std::collections::HashMap;
use std::collections::hash_map::{Occupied, Vacant};
use graphics;

use Texture;

pub type FontSize = u32;

pub type Character = graphics::character::Character<Texture>;

/// A struct used for caching rendered font.
pub struct GlyphCache {
    /// The font face.
    pub face: freetype::Face,
    data: HashMap<FontSize, HashMap<char, Character>>,
}

impl GlyphCache {

    /// Constructor for a GlyphCache.
    pub fn new(font: &Path) -> Result<GlyphCache, Error> {
        let freetype = match freetype::Library::init() {
            Ok(freetype) => freetype,
            Err(why) => return Err(Error::FreetypeError(why)),
        };
        let font_str = match font.as_str() {
            Some(font_str) => font_str,
            None => return Err(Error::FreetypeError(MissingFontField)),
        };
        let face = match freetype.new_face(font_str, 0) {
            Ok(face) => face,
            Err(why) => return Err(Error::FreetypeError(why)),
        };
        Ok(GlyphCache {
            face: face,
            data: HashMap::new(),
        })
    }

    /// Return a reference to a `Character`. If there is not yet a `Character` for
    /// the given `FontSize` and `char`, load the `Character`.
    pub fn get_character(&mut self, size: FontSize, ch: char) -> &Character {
        match {
            match self.data.entry(size) {
                Vacant(entry) => entry.set(HashMap::new()),
                Occupied(entry) => entry.into_mut(),
            }
        }.contains_key(&ch) {
            true => &self.data[size][ch],
            false => { self.load_character(size, ch); &self.data[size][ch] }
        }
    }

    /// Load a `Character` from a given `FontSize` and `char`.
    fn load_character(&mut self, size: FontSize, ch: char) {
        self.face.set_pixel_sizes(0, size).unwrap();
        self.face.load_char(ch as ffi::FT_ULong, freetype::face::DEFAULT).unwrap();
        let glyph = self.face.glyph().get_glyph().unwrap();
        let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::RenderMode::Normal, None)
            .unwrap();
        let bitmap = bitmap_glyph.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                 bitmap.width() as u32,
                                                 bitmap.rows() as u32).unwrap();
        let glyph_size = glyph.advance();
        self.data[size].insert(ch, Character {
            offset: [
                    bitmap_glyph.left() as f64, 
                    bitmap_glyph.top() as f64
                ],
            size: [
                    (glyph_size.x >> 16) as f64,
                    (glyph_size.y >> 16) as f64
                ],
            texture: texture,
        });
    }

}
