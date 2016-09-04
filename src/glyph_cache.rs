//! Glyph caching

use {rusttype, graphics, Texture, TextureSettings};
use std::collections::HashMap;
use graphics::types::Scalar;

extern crate fnv;
use self::fnv::FnvHasher;
use std::hash::BuildHasherDefault;

use std::path::Path;
use std::io::Read;
use std::fs::File;
use error::Error;

pub use graphics::types::FontSize;

/// The type alias for font characters.
pub type Character<'a> = graphics::character::Character<'a, Texture>;

/// A struct used for caching rendered font.
pub struct GlyphCache<'a> {
    /// The font.
    pub font: rusttype::Font<'a>,
    // Maps from fontsize and character to offset, size and texture.
    data: HashMap<(FontSize, char),
                  ([Scalar; 2], [Scalar; 2], Texture),
                  BuildHasherDefault<FnvHasher>>,
}

impl<'a> GlyphCache<'a> {
    /// Constructor for a GlyphCache.
    pub fn new<P>(font: P) -> Result<GlyphCache<'static>, Error>
        where P: AsRef<Path>
    {
        let fnv = BuildHasherDefault::<FnvHasher>::default();
        let mut file = try!(File::open(font));
        let mut file_buffer = Vec::new();
        try!(file.read_to_end(&mut file_buffer));

        let collection = rusttype::FontCollection::from_bytes(file_buffer);
        let font = collection.into_font().unwrap();
        Ok(GlyphCache {
            font: font,
            data: HashMap::with_hasher(fnv),
        })
    }

    /// Creates a GlyphCache for a font stored in memory.
    pub fn from_bytes(font: &'a [u8]) -> Result<GlyphCache<'a>, Error> {
        let fnv = BuildHasherDefault::<FnvHasher>::default();
        let collection = rusttype::FontCollection::from_bytes(font);
        let font = collection.into_font().unwrap();
        Ok(GlyphCache {
            font: font,
            data: HashMap::with_hasher(fnv),
        })
    }

    /// Get a `Character` from cache, or load it if not there.
    fn get(&mut self, size: FontSize, ch: char) -> &([Scalar; 2], [Scalar; 2], Texture) {
        // Create a `Character` from a given `FontSize` and `char`.
        fn create_character(font: &rusttype::Font,
                            size: FontSize,
                            ch: char)
                            -> ([Scalar; 2], [Scalar; 2], Texture) {
            let size = ((size as f32) * 1.333).round() as u32;
            let glyph = font.glyph(ch).unwrap_or(font.glyph(rusttype::Codepoint(0))
                .unwrap_or(font.glyph('\u{FFd}').unwrap()));
            let glyph = glyph.scaled(rusttype::Scale::uniform(size as f32));
            let h_metrics = glyph.h_metrics();
            let pixel_bounding_box = glyph.exact_bounding_box().unwrap_or(rusttype::Rect {
                min: rusttype::Point { x: 0.0, y: 0.0 },
                max: rusttype::Point { x: 0.0, y: 0.0 },
            });
            let pixel_bb_width = pixel_bounding_box.width();
            let pixel_bb_height = pixel_bounding_box.height();

            let mut image_buffer = Vec::new();
            image_buffer.resize((pixel_bb_width * pixel_bb_height) as usize, 0);
            glyph.positioned(rusttype::point(0.0, 0.0)).draw(|x, y, v| {
                let pos = (x + y * (pixel_bb_width as u32)) as usize;
                image_buffer[pos] = (255.0 * v) as u8;
            });
            let texture = Texture::from_memory_alpha(&image_buffer,
                                                     pixel_bb_width as u32,
                                                     pixel_bb_height as u32,
                                                     &TextureSettings::new())
                .unwrap();
            ([pixel_bounding_box.min.x as Scalar, -pixel_bounding_box.min.y as Scalar],
             [h_metrics.advance_width as Scalar, 0 as Scalar],
             texture)
        }

        let font = &self.font;// necessary to borrow-check
        self.data
            .entry((size, ch))
            .or_insert_with(|| create_character(font, size, ch))
    }

    /// Load all characters in the `chars` iterator for `size`
    pub fn preload_chars<I>(&mut self, size: FontSize, chars: I)
        where I: Iterator<Item = char>
    {
        for ch in chars {
            self.get(size, ch);
        }
    }

    /// Load all the printable ASCII characters for `size`. Includes space.
    pub fn preload_printable_ascii(&mut self, size: FontSize) {
        // [0x20, 0x7F) contains all printable ASCII characters ([' ', '~'])
        self.preload_chars(size, (0x20u8..0x7F).map(|ch| ch as char));
    }

    /// Return `ch` for `size` if it's already cached. Don't load.
    /// See the `preload_*` functions.
    pub fn opt_character(&self, size: FontSize, ch: char) -> Option<Character> {
        self.data.get(&(size, ch)).map(|&(offset, size, ref texture)| {
            Character {
                offset: offset,
                size: size,
                texture: texture,
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
            texture: texture,
        };
    }
}
