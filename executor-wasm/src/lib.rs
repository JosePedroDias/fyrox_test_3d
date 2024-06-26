//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use fyrox::dpi::LogicalSize;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;
use fyrox::core::wasm_bindgen::{self, prelude::*};

use fyrox_test_3d::GameConstructor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

fn custom_panic_hook(info: &std::panic::PanicInfo) {
    let mut msg = info.to_string();
    msg.push_str("\n\nStack:\n\n");
    let e = Error::new();
    let stack = e.stack();
    msg.push_str(&stack);
    msg.push_str("\n\n");
    error(msg);
}

#[inline]
pub fn set_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(custom_panic_hook));
    });
}

#[wasm_bindgen]
pub fn main() {
    set_panic_hook();
    //let mut executor = Executor::new();
    
    let mut window_attributes = WindowAttributes::default();
    window_attributes.inner_size = Some(LogicalSize::new(1280.0, 720.0).into());
    window_attributes.resizable = true;
    window_attributes.title = "3d game".to_string();
    let mut executor = Executor::from_params(
        EventLoop::new().unwrap(),
        GraphicsContextParams {
            window_attributes,
            vsync: true,
        },
    );
    
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}