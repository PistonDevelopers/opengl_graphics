extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use graphics::draw_state::Blend;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::{OpenGL, Sdl2Window};
use std::path::Path;

fn main() {
    println!("Press A to change blending");
    println!("Press S to change clip inside/out");

    let opengl = OpenGL::V3_2;
    let (w, h) = (640, 480);
    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: draw_state", [w, h])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut clip_inside = true;
    let blends = [
        Blend::Alpha,
        Blend::Add,
        Blend::Invert,
        Blend::Multiply,
        Blend::Lighter,
    ];
    let mut blend = 0;
    let rust_logo =
        Texture::from_path(&Path::new("./assets/rust.png"), &TextureSettings::new()).unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            use graphics::*;

            gl.draw(args.viewport(), |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                    [0.0, 0.0, 100.0, 100.0],
                    &c.draw_state,
                    c.transform,
                    g,
                );

                let draw_state = c.draw_state.blend(blends[blend]);
                Rectangle::new([0.5, 1.0, 0.0, 0.3]).draw(
                    [50.0, 50.0, 100.0, 100.0],
                    &draw_state,
                    c.transform,
                    g,
                );

                let transform = c.transform.trans(100.0, 100.0);
                // Clip rectangle from upper left corner.
                let clipped = c.draw_state.scissor([100, 100, 100, 100]);
                Image::new().draw(&rust_logo, &clipped, transform, g);

                let transform = c.transform.trans(200.0, 200.0);
                Ellipse::new([1.0, 0.0, 0.0, 1.0]).draw(
                    [0.0, 0.0, 50.0, 50.0],
                    &DrawState::new_clip(),
                    transform,
                    g,
                );
                Image::new().draw(
                    &rust_logo,
                    &if clip_inside {
                        DrawState::new_inside()
                    } else {
                        DrawState::new_outside()
                    },
                    transform,
                    g,
                );
            });
        }

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            blend = (blend + 1) % blends.len();
            println!("Changed blending to {:?}", blends[blend]);
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            clip_inside = !clip_inside;
            if clip_inside {
                println!("Changed to clip inside");
            } else {
                println!("Changed to clip outside");
            }
        }
    }
}
