#[cfg(feature = "glow")]
use graphics::{clear, Graphics, Rectangle, Transformed as _};
#[cfg(feature = "glow")]
use graphics::{image, text};

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
    let w = 600;
    let h = 600;

    #[cfg(not(target_arch = "wasm32"))]
    let (gl, window, event_loop, gl_surface, gl_context) = {
        unsafe {
            let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
            let window_builder = winit::window::WindowAttributes::new()
                .with_title("Hello Rust!")
                .with_inner_size(winit::dpi::LogicalSize::new(w as f64, w as f64));
            /*
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            */

            let template = glutin::config::ConfigTemplateBuilder::new();

            let display_builder = glutin_winit::DisplayBuilder::new().with_window_attributes(Some(window_builder));

            use glutin::context::NotCurrentGlContext;
            let (window, gl_config) = display_builder
                .build(&event_loop, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            use glutin::config::GlConfig;

                            if config.num_samples() > accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();
          
            use winit::raw_window_handle::HasRawWindowHandle; 
            let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle().unwrap());
 
            use glutin::display::{GetGlDisplay, GlDisplay};
            let gl_display = gl_config.display();
            use glutin::context::ContextAttributesBuilder;
            use glutin::context::ContextApi;
            let context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                    major: 4,
                    minor: 1,
                })))
                .build(raw_window_handle);

            let not_current_gl_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap();

            let window = window.unwrap();

            use glutin_winit::GlWindow;
            let attrs = window.build_surface_attributes(Default::default()).unwrap();
            let gl_surface = gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap();
            let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

            /*
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            */
            let gl = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));
            (gl, window, event_loop, gl_surface, gl_context)
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
    let mut gl = GlGraphics::new(OpenGL::V4_2);

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

    let scale = window.scale_factor();
    let viewport = Viewport {
        rect: [0, 0, (scale * w as f64) as i32, (scale * h as f64) as i32],
        draw_size: [w as u32, h as u32],
        window_size: [w.into(), h.into()],
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        use winit::event::{Event, WindowEvent};
        use winit::event_loop::ControlFlow;

        let _ = event_loop.run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        render(&mut gl, viewport, &rust_logo, &mut glyph_cache);
                        // window.swap_buffers().unwrap();
                        use glutin::prelude::GlSurface;
                        gl_surface.swap_buffers(&gl_context).unwrap();
                    }
                    _ => (),
                }
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
        let c = c.zoom(2.0);
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
                &c.draw_state,
                c.transform.trans(100.0, 300.0),
                g,
            )
            .unwrap();
    });
}
