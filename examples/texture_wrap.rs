extern crate graphics;
extern crate image as im;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use opengl_graphics::{GlGraphics, Texture, TextureSettings, Wrap};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::{OpenGL, Sdl2Window};
use std::path::Path;

fn main() {
    println!("Press U to change the texture wrap mode for the u coordinate");
    println!("Press V to change the texture wrap mode for the v coordinate");

    let opengl = OpenGL::V3_2;
    let (w, h) = (640, 480);
    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: texture_wrap", [w, h])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Set up wrap modes
    let wrap_modes = [
        Wrap::ClampToEdge,
        Wrap::ClampToBorder,
        Wrap::Repeat,
        Wrap::MirroredRepeat,
    ];
    let mut ix_u = 0;
    let mut ix_v = 0;
    let mut texture_settings = TextureSettings::new();
    texture_settings.set_border_color([0.0, 0.0, 0.0, 1.0]);

    // Set up texture
    let path = Path::new("./assets/rust.png");
    let img = match im::open(path) {
        Ok(img) => img,
        Err(e) => {
            panic!("Could not load '{:?}': {:?}", path.file_name().unwrap(), e);
        }
    };
    let img = match img {
        im::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba(),
    };
    let mut rust_logo = Texture::from_image(&img, &texture_settings);

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw(args.viewport(), |_c, g| {
                clear([1.0; 4], g);
                let points = [[0.5, 0.5], [-0.5, 0.5], [-0.5, -0.5], [0.5, -0.5]];
                // (0, 1, 2) and (0, 2, 3)
                let uvs = [
                    [4.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 4.0],
                    [4.0, 0.0],
                    [0.0, 4.0],
                    [4.0, 4.0],
                ];
                let mut verts = [[0.0, 0.0]; 6];
                let indices_points: [usize; 6] = [0, 1, 2, 0, 2, 3];
                for (ixv, &ixp) in (0..6).zip((&indices_points).into_iter()) {
                    verts[ixv] = points[ixp];
                }
                g.tri_list_uv(&DrawState::new_alpha(), &[1.0; 4], &rust_logo, |f| {
                    f(&verts, &uvs)
                });
            });
        }

        if let Some(Button::Keyboard(Key::U)) = e.press_args() {
            ix_u = (ix_u + 1) % wrap_modes.len();
            texture_settings.set_wrap_u(wrap_modes[ix_u]);
            rust_logo = Texture::from_image(&img, &texture_settings);
            println!(
                "Changed texture wrap mode for u coordinate to: {:?}",
                wrap_modes[ix_u]
            );
        }

        if let Some(Button::Keyboard(Key::V)) = e.press_args() {
            ix_v = (ix_v + 1) % wrap_modes.len();
            texture_settings.set_wrap_v(wrap_modes[ix_v]);
            rust_logo = Texture::from_image(&img, &texture_settings);
            println!(
                "Changed texture wrap mode for v coordinate to: {:?}",
                wrap_modes[ix_v]
            );
        }
    }
}
