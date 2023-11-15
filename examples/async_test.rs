extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston::window::Window;
use piston::Size;
use opengl_graphics::*;
use sdl2_window::Sdl2Window;

use futures::stream::FuturesUnordered;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: async test", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    events.set_ups(10);

    let n = 100_000;
    let size = window.size();
    let mut ps: Vec<[f64; 2]> = (0..n)
        .map(|_| [rand::random::<f64>() * size.width, rand::random::<f64>() * size.height])
        .collect();
    let mut task: FuturesUnordered<_> = FuturesUnordered::new();
    let mut cursor = [0.0, 0.0];
    let mut time = 0.0;
    while let Some(e) = events.async_next(&mut window).await {
        use graphics::*;

        while let Some(pos) = task.next().await {
            ps.push(pos);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);
                for pos in &ps {
                    Rectangle::new([1.0, 0.0, 0.0, 0.05])
                        .draw([pos[0], pos[1], 2.0, 2.0], &c.draw_state, c.transform, g);
                }
            });
        }

        if let Some(args) = e.update_args() {
            time += args.dt;
            let size = window.size();
            task.extend(ps.into_iter().map(|pos| update(time, args.dt, cursor, pos, size)));
            ps = vec![];
        }

        if let Some(pos) = e.mouse_cursor_args() {
            cursor = pos;
        }
    }
}

async fn update(time: f64, dt: f64, cursor: [f64; 2], pos: [f64; 2], size: Size) -> [f64; 2] {
    let old_pos = pos;
    let mut pos = pos;

    let dir = [pos[0] - cursor[0], pos[1] - cursor[1]];
    let dir_len = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
    let dir_len = if dir_len <= 0.001 {1.0} else {dir_len};
    let trigger = if (0.1 * (dir_len + time)).sin() < 0.1 {0.1} else {-0.1};
    let speed = 10.0;

    pos[0] += speed * dt * (
        2.0 * rand::random::<f64>() - 1.0 + trigger * rand::random::<f64>() * dir[0] / dir_len
    );
    pos[1] += speed * dt * (
        2.0 * rand::random::<f64>() - 1.0 + trigger * rand::random::<f64>() * dir[1] / dir_len
    );
    if pos[0] <= 0.0 || pos[0] >= size.width {
        pos[0] = old_pos[0];
    }
    if pos[1] <= 0.0 || pos[1] >= size.height {
        pos[1] = old_pos[1];
    }
    pos
}
