//! Glyph caching

use { freetype, graphics };
use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };
use std::rc::Rc;

use error::Error;
use Texture;

/// The type alias for the font size.
pub type FontSize = u32;

/// The type alias for font characters.
pub type Character = graphics::character::Character<Texture>;

/// A struct used for caching rendered font.
#[derive(Clone)]
pub struct GlyphCache {
    /// The font face.
    pub face: freetype::Face,
    data: HashMap<FontSize, HashMap<char, Rc<Character>>>,
}

impl GlyphCache {

    /// Constructor for a GlyphCache.
    pub fn new(font: &Path) -> Result<Self, Error> {
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
            data: HashMap::new(),
        })
    }

    /// Creates a GlyphCache for a font stored in memory.
    pub fn from_bytes(font: &[u8]) -> Result<Self, Error> {
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
            data: HashMap::new()
        })
    }

    /// Load a `Character` from a given `FontSize` and `char`.
    fn load_character(&mut self, size: FontSize, ch: char) {
        // Don't load glyph twice
        if self.data.get(&size)
            .map(|entry| entry.contains_key(&ch))
            .unwrap_or(false) { return }

        self.face.set_pixel_sizes(0, size).unwrap();
        self.face.load_char(ch as usize, freetype::face::DEFAULT).unwrap();
        let glyph = self.face.glyph().get_glyph().unwrap();
        let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::RenderMode::Normal, None)
            .unwrap();
        let bitmap = bitmap_glyph.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                 bitmap.width() as u32,
                                                 bitmap.rows() as u32).unwrap();
        let glyph_size = glyph.advance();
        self.data[size].insert(ch, Rc::new(Character {
            offset: [
                    bitmap_glyph.left() as f64,
                    bitmap_glyph.top() as f64
                ],
            size: [
                    (glyph_size.x >> 16) as f64,
                    (glyph_size.y >> 16) as f64
                ],
            texture: texture,
        }));
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
            self.load_character(size, ch);
        }
    }

    /// Load all the printable ASCII characters for `size`. Includes space.
    pub fn preload_printable_ascii(&mut self, size: FontSize) {
        // [0x20, 0x7F) contains all printable ASCII characters ([' ', '~'])
        self.preload_chars(size, (0x20u8 .. 0x7F).map(|ch| ch as char));
    }

    /// Return `ch` for `size` if it's already cached. Don't load.
    /// See the `preload_*` functions.
    pub fn opt_character(&self, size: FontSize, ch: char) -> Option<&Character> {
        use std::borrow::Borrow;

        self.data.get(&size).and_then(|entry| entry.get(&ch).map(|e| e.borrow()))
    }
}

impl graphics::character::CharacterCache for GlyphCache {
    type Texture = Texture;

    fn character(&mut self, size: FontSize, ch: char) -> &Character {
        match {
            match self.data.entry(size) {
                Vacant(entry) => entry.insert(HashMap::new()),
                Occupied(entry) => entry.into_mut(),
            }
        }.contains_key(&ch) {
            true => &self.data[size][ch],
            false => { self.load_character(size, ch); &self.data[size][ch] }
        }
    }
}
