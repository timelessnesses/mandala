use std::{cell::RefCell, rc::Rc};

fn main() {
    if std::env::args().nth(1) == Some("multi".to_string()) {
        mandala::multi_threaded::multi_threaded_main().unwrap();
    } else {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();

        let window = match video_ctx
            .window("Mandala", 800, 600)
            .position_centered()
            .opengl()
            .build()
        {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err),
        };

        let canvas = match window.into_canvas().present_vsync().build() {
            Ok(canvas) => canvas,
            Err(err) => panic!("failed to create canvas: {}", err),
        };

        let ctx = Rc::new(RefCell::new(ctx));
        let canvas = Rc::new(RefCell::new(canvas));

        #[cfg(not(target_arch = "wasm32"))]
        mandala::main_loop(Rc::clone(&ctx), Rc::clone(&canvas)).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            mandala::emscripten::set_main_loop_callback(move || {
                mandala::main_loop(Rc::clone(&ctx), Rc::clone(&canvas)).unwrap();
            });
        }
    }
}
