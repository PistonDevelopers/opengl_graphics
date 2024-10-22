extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use graphics::draw_state::{Blend, Stencil};
use graphics::DrawState;
use opengl_graphics::GlGraphics;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::{OpenGL, Sdl2Window};

fn main() {
    let opengl = OpenGL::V3_2;
    let (w, h) = (640, 480);
    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: nested_clipping", [w, h])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    let increment = DrawState::new_increment();
    let inside_level1 = DrawState {
        blend: Some(Blend::Alpha),
        stencil: Some(Stencil::Inside(1)),
        scissor: None,
    };
    let inside_level2 = DrawState {
        blend: Some(Blend::Alpha),
        stencil: Some(Stencil::Inside(2)),
        scissor: None,
    };
    let inside_level3 = DrawState {
        blend: Some(Blend::Alpha),
        stencil: Some(Stencil::Inside(3)),
        scissor: None,
    };
    let mut clip = true;
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            use graphics::*;

            gl.draw(args.viewport(), |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);

                if clip {
                    Rectangle::new([1.0; 4]).draw(
                        [10.0, 10.0, 200.0, 200.0],
                        &increment,
                        c.transform,
                        g,
                    );
                    Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                        [10.0, 10.0, 200.0, 200.0],
                        &inside_level1,
                        c.transform,
                        g,
                    );

                    Rectangle::new([1.0; 4]).draw(
                        [100.0, 100.0, 200.0, 200.0],
                        &increment,
                        c.transform,
                        g,
                    );
                    Rectangle::new([0.0, 0.0, 1.0, 1.0]).draw(
                        [100.0, 100.0, 200.0, 200.0],
                        &inside_level2,
                        c.transform,
                        g,
                    );

                    Rectangle::new([1.0; 4]).draw(
                        [100.0, 100.0, 200.0, 200.0],
                        &increment,
                        c.transform,
                        g,
                    );
                    Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                        [50.0, 50.0, 200.0, 100.0],
                        &inside_level3,
                        c.transform,
                        g,
                    );
                } else {
                    Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                        [10.0, 10.0, 200.0, 200.0],
                        &c.draw_state,
                        c.transform,
                        g,
                    );

                    Rectangle::new([0.0, 0.0, 1.0, 1.0]).draw(
                        [100.0, 100.0, 200.0, 200.0],
                        &c.draw_state,
                        c.transform,
                        g,
                    );

                    Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                        [50.0, 50.0, 200.0, 100.0],
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            });
        }
        if e.press_args().is_some() {
            clip = !clip;
        }
    }
}
