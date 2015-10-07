extern crate graphics;
extern crate opengl_graphics;
extern crate image;
extern crate piston;
extern crate sdl2_window;

use std::path::Path;
use opengl_graphics::{ GlGraphics, Texture };
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::{ Sdl2Window, OpenGL };
use graphics::draw_state::BlendPreset;

fn main() {
    println!("Set image `default-features = true` in Cargo.toml to test");
    println!("Press A to change blending");
    println!("Press S to change clip inside/out");

    let opengl = OpenGL::V3_2;
    let (w, h) = (640, 480);
    let window: Sdl2Window =
        WindowSettings::new("glium_graphics: image_test", [w, h])
        .exit_on_esc(true).build().unwrap();

    let mut blend = BlendPreset::Alpha;
    let mut clip_inside = true;
    let rust_logo = Texture::from_path(&Path::new("./assets/rust.png")).unwrap();
    let mut gl = GlGraphics::new(opengl);
    for e in window.events() {
        if let Some(args) = e.render_args() {
            use graphics::*;

            gl.draw(args.viewport(), |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                g.clear_stencil(0);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);

                let draw_state = c.draw_state.blend(blend);
                Rectangle::new([0.5, 1.0, 0.0, 0.3])
                    .draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);

                let transform = c.transform.trans(100.0, 100.0);
                // Compute clip rectangle from upper left corner.
                let (clip_x, clip_y, clip_w, clip_h) = (100, 100, 100, 100);
                let (clip_x, clip_y, clip_w, clip_h) =
                    (clip_x, c.viewport.unwrap().draw_size[1] as u16 - clip_y - clip_h, clip_w, clip_h);
                let clipped = c.draw_state.scissor(clip_x, clip_y, clip_w, clip_h);
                Image::new().draw(&rust_logo, &clipped, transform, g);

                let transform = c.transform.trans(200.0, 200.0);
                Ellipse::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 50.0, 50.0], clip_draw_state(), transform, g);
                Image::new().draw(&rust_logo,
                    if clip_inside { inside_draw_state() }
                    else { outside_draw_state() },
                    transform, g);
            });
        }

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            blend = match blend {
                BlendPreset::Alpha => BlendPreset::Add,
                BlendPreset::Add => BlendPreset::Multiply,
                BlendPreset::Multiply => BlendPreset::Invert,
                BlendPreset::Invert => BlendPreset::Alpha,
            };
            println!("Changed blending to {:?}", blend);
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
