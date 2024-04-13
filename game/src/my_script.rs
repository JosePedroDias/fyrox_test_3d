#![allow(unused)]

use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*, log::Log},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "159d6d6b-73f9-4fdc-9a3f-771259d349d7")]
#[visit(optional)]
pub struct MyScript {
    // Add fields here.
}

impl ScriptTrait for MyScript {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        Log::info(format!("MyScript on_init()"));
    }
    
    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
    }
}
    