extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston::window::{WindowSettings, Size};
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::*;
use opengl_graphics::GlyphCache;
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::V3_2;
    let size = Size {
        width: 500,
        height: 300,
    };
    let ref mut window: Sdl2Window = WindowSettings::new("opengl_graphics: text_test", size)
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;

                clear([1.0; 4], g);
                text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32).draw("Hello opengl_graphics!",
                                                                     &mut glyph_cache,
                                                                     &DrawState::default(),
                                                                     c.transform
                                                                         .trans(10.0, 100.0),
                                                                     g).unwrap();
            });
        }
    }
}
