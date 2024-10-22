extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;
use std::path::Path;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window =
        WindowSettings::new("opengl_graphics: colored_image_test", [300, 300])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

    let rust_logo = Texture::from_path(
        &Path::new("./assets/rust-white.png"),
        &TextureSettings::new(),
    )
    .unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::triangulation::{tx, ty};

                let transform = c.transform.trans(0.0, 0.0);

                let tr = |p: [f64; 2]| [tx(transform, p[0], p[1]), ty(transform, p[0], p[1])];

                clear([1.0; 4], g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                    [0.0, 0.0, 100.0, 100.0],
                    &c.draw_state,
                    c.transform,
                    g,
                );
                Rectangle::new([0.0, 1.0, 0.0, 0.3]).draw(
                    [50.0, 50.0, 100.0, 100.0],
                    &c.draw_state,
                    c.transform,
                    g,
                );
                g.tri_list_uv_c(&c.draw_state, &rust_logo, |f| {
                    (f)(
                        &[
                            tr([0.0, 0.0]),
                            tr([300.0, 0.0]),
                            tr([0.0, 300.0]),
                            tr([300.0, 0.0]),
                            tr([0.0, 300.0]),
                            tr([300.0, 300.0]),
                        ],
                        &[
                            [0.0, 0.0],
                            [1.0, 0.0],
                            [0.0, 1.0],
                            [1.0, 0.0],
                            [0.0, 1.0],
                            [1.0, 1.0],
                        ],
                        &[
                            [1.0, 0.0, 0.0, 1.0],
                            [0.0, 1.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0, 1.0],
                            [0.0, 00.0, 0.0, 1.0],
                            [0.0, 00.0, 0.0, 1.0],
                            [0.0, 00.0, 0.0, 1.0],
                        ],
                    )
                });
            });
        }
    }
}
