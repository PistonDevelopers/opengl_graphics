use std::{path::Path, sync::Arc};

use graphics::{image, text, DrawState};
use graphics::{clear, Graphics, Rectangle, Transformed as _};
use opengl_graphics::*;
use shader_version::OpenGL;
use texture::TextureSettings;
use viewport::Viewport;

fn main() {
    let (gl, window, event_loop) = {
        unsafe {
            let event_loop = glutin::event_loop::EventLoop::new();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Hello Rust!")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            (gl, window, event_loop)
        }
    };
    let gl = Arc::new(gl);

    opengl_graphics::gl::init(gl);
    let mut gl = GlGraphics::new(OpenGL::V4_0);

    let rust_logo =
        Texture::from_path(&Path::new("./assets/rust.png"), &TextureSettings::new()).unwrap();

    let mut glyph_cache =
        GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

    use glutin::event::{Event, WindowEvent};
    use glutin::event_loop::ControlFlow;

    let mut viewport = Viewport {
        rect: [0, 0, 1024, 768],
        draw_size: [1, 1],
        window_size: [1.0, 1.0],
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                return;
            }
            Event::MainEventsCleared => {
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                gl.clear_color([0.5, 0.5, 0.5, 1.0]);
                gl.draw(viewport, |c, g| {
                    let transform = c.transform.trans(100.0, 100.0);

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
                    image(&rust_logo, transform, g);

                    text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32)
                        .draw(
                            "Hello opengl_graphics!",
                            &mut glyph_cache,
                            &DrawState::default(),
                            c.transform.trans(10.0, 100.0),
                            g,
                        )
                        .unwrap();
                });
                window.swap_buffers().unwrap();
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    window.resize(*physical_size);
                    viewport.rect = [
                        0,
                        0,
                        physical_size.width as i32,
                        physical_size.height as i32,
                    ];
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
