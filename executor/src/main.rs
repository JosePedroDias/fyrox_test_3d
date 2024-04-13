//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use fyrox::window::WindowAttributes;
use fyrox::dpi::LogicalSize;
use fyrox::event_loop::EventLoop;
use fyrox::engine::GraphicsContextParams;

use fyrox_test_3d::GameConstructor;

fn main() {
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