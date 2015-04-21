extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use piston::event::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Rc::new(RefCell::new(
        Sdl2Window::new(
            opengl,
            WindowSettings::new(
                "gfx_graphics: image_test",
                [300, 300]
            )
            .exit_on_esc(true)
        )
    ));

    let rust_logo = Texture::from_path(&Path::new("./assets/rust.png")).unwrap();
    let mut gl = Gl::new(opengl);
    for e in window.events() {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let transform = c.transform.trans(100.0, 100.0);

                clear([1.0; 4], g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0],
                          &c.draw_state,
                          c.transform,
                          g);
                Rectangle::new([0.0, 1.0, 0.0, 0.3])
                    .draw([50.0, 50.0, 100.0, 100.0],
                          &c.draw_state,
                          c.transform,
                          g);
                image(&rust_logo, transform, g);
            });
        }
    }
}
