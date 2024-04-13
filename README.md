https://fyrox-book.github.io/beginning/scripting.html
    cargo install fyrox-template
        fyrox-template init --name fyrox_test_2d --style 2d
        or
        fyrox-template init --name fyrox_test_3d--style 3d
        cd <folder>
        cargo run --package editor --release
        cargo run --package executor --release

https://fyrox-book.github.io/scripting/script.html
    fyrox-template script --name MyScript
    add to game/src/lib.rs:
        pub mod my_script;
        //on the register fn
        context.serialization_context.script_constructors.add::<MyScript>("My Script");
        
    https://fyrox-book.github.io/scripting/script.html#script-registration

https://fyrox-book.github.io/shipping/wasm.html

in each file nagging about unused vars, at root file level:
```rust
#![allow(unused)]
```

logging is available under fyrox::log::Log
Log::info(...)

https://fyrox-book.github.io/scene/base_node.html


https://github.com/FyroxEngine/Fyrox/tree/master/examples
https://github.com/FyroxEngine/Fyrox-demo-projects https://fyrox.rs/examples.html

https://bocksdincoding.com/
https://bocksdincoding.com/blog/game-development-with-fyrox-and-rust-pt-2
https://github.com/bocksdin/blog-fyrox-game-dev-tutorial/tree/player-controller