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
        
    https://fyrox-book.github.io/scripting/script.html#script-registration


in each file nagging about unused vars, at root file level:
```rust
#![allow(unused)]
```
