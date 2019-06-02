extern crate rand;
extern crate graphics;
extern crate opengl_graphics;
extern crate image as im;
extern crate sdl2_window;
extern crate piston;

use sdl2_window::Sdl2Window;
use graphics::*;
use opengl_graphics::*;
use piston::event_loop::*;
use piston::window::*;
use piston::input::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let texture_count = 1024;
    let frames = 200;
    let size = 32.0;

    let mut window: Sdl2Window = WindowSettings::new("texture_swap", [1024; 2])
        .graphics_api(opengl).build().unwrap();

    let textures = {
        (0..texture_count).map(|_| {
            let mut img = im::ImageBuffer::new(2, 2);
            for y in 0..2 {
                for x in 0..2 {
                    img.put_pixel(x, y,
                        im::Rgba([rand::random(), rand::random(), rand::random(), 255]));
                }
            }
            Texture::from_image(
                &img,
                &TextureSettings::new()
            )
        }).collect::<Vec<Texture>>()
    };

    let mut positions = (0..texture_count)
        .map(|_| (rand::random(), rand::random()))
        .collect::<Vec<(f64, f64)>>();

    let mut counter = 0;
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().bench_mode(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            counter += 1;
            if counter > frames { break; }
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                for p in &mut positions {
                    let (x, y) = *p;
                    *p = (x + (rand::random::<f64>() - 0.5) * 0.01,
                          y + (rand::random::<f64>() - 0.5) * 0.01);
                }
                for i in 0..texture_count {
                    let p = positions[i];
                    image(&textures[i], c.transform
                        .trans(p.0 * 1024.0, p.1 * 1024.0).zoom(size), g);
                }
            });
        }
    }
}
