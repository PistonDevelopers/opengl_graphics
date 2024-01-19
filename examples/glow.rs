#[cfg(feature = "glow")]
use graphics::{clear, Graphics, Rectangle, Transformed as _};
#[cfg(feature = "glow")]
use graphics::{image, text, DrawState};

#[cfg(feature = "glow")]
use opengl_graphics::*;

#[cfg(feature = "glow")]
use std::sync::Arc;
#[cfg(feature = "glow")]
use viewport::Viewport;

#[cfg(not(feature = "glow"))]
fn main() {}

///
/// Run with glutin
/// ```
/// cargo run --features glow --example glow
/// ```
///
/// Run with web_sys
/// ```
/// cargo build --features glow --target wasm32-unknown-unknown --example glow
/// mkdir -p target/glow
/// wasm-bindgen target/wasm32-unknown-unknown/debug/examples/glow.wasm --out-dir target/glow --target web
/// cp assets/glow.html target/glow/index.html
/// cd target/glow
/// cargo install cargo-server
/// cargo server --open
/// ```
#[cfg(feature = "glow")]
fn main() {


    #[cfg(not(target_arch = "wasm32"))]
    let (gl, window, event_loop) = {
        unsafe {
            let event_loop = glutin::event_loop::EventLoop::new();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Hello Rust!")
                .with_inner_size(glutin::dpi::LogicalSize::new(600.0, 600.0));
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

    #[cfg(target_arch = "wasm32")]
    let gl = {
        use wasm_bindgen::JsCast;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let gl = glow::Context::from_webgl2_context(webgl2_context);
        gl
    };

    opengl_graphics::set_context(Arc::new(gl));
    let mut gl = GlGraphics::new(OpenGL::V2_1);

    #[cfg(not(target_arch = "wasm32"))]
    let (rust_logo, mut glyph_cache) = {
        let rust_logo = Texture::from_path(
            &std::path::Path::new("assets/rust.png"),
            &TextureSettings::new(),
        )
        .unwrap();
        let glyph_cache =
            GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        (rust_logo, glyph_cache)
    };

    #[cfg(target_arch = "wasm32")]
    let (rust_logo, mut glyph_cache) = {
        let mut setting = TextureSettings::new();
        setting.set_convert_gamma(true);
        let rust_logo =
            Texture::from_bytes(include_bytes!("../assets/rust.png"), &setting).unwrap();
        let glyph_cache = GlyphCache::from_bytes(
            include_bytes!("../assets/FiraSans-Regular.ttf"),
            (),
            setting,
        )
        .unwrap();
        (rust_logo, glyph_cache)
    };

    let mut viewport = Viewport {
        rect: [0, 0, 600, 600],
        draw_size: [1, 1],
        window_size: [1.0, 1.0],
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

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
                    render(&mut gl, viewport, &rust_logo, &mut glyph_cache);
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

    #[cfg(target_arch = "wasm32")]
    {
        render(&mut gl, viewport, &rust_logo, &mut glyph_cache);
    }
}

#[cfg(feature = "glow")]
fn render(
    gl: &mut GlGraphics,
    viewport: Viewport,
    logo: &opengl_graphics::Texture,
    glyph_cache: &mut GlyphCache,
) {
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
        image(logo, transform.rot_deg(45.0), g);

        text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32)
            .draw(
                "Hello opengl_graphics!",
                glyph_cache,
                &DrawState::default(),
                c.transform.trans(10.0, 100.0),
                g,
            )
            .unwrap();
    });
}
