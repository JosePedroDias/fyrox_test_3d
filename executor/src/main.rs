//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
//use fyrox::event_loop::EventLoop;
//use fyrox::engine::GraphicsContextParams;
//use fyrox::window::WindowAttributes;
//use fyrox::dpi;

use fyrox_test_3d::GameConstructor;

fn main() {
    let mut executor = Executor::new();
    
    // let mut executor = Executor::from_params(
    //     EventLoop::new().unwrap(),
    //     GraphicsContextParams {
    //         window_attributes: WindowAttributes {
    //             title: "My Game".to_string(),
    //             resizable: true,
    //             min_inner_size: fyrox::dpi::Size::Physical(fyrox::dpi::Size::(400, 300)),
    //         },
    //         vsync: true,
    //     },
    // );
    
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}