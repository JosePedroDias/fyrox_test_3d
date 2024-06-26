#![allow(unused)]

use std::{process::exit, borrow::Borrow};

use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*, log::Log},
    event::{Event, WindowEvent, ElementState, DeviceId, DeviceEvent, MouseButton},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
    keyboard::{PhysicalKey, KeyCode}, scene::mesh::Mesh
};

use fyrox::core::algebra::Vector3;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "159d6d6b-73f9-4fdc-9a3f-771259d349d7")]
#[visit(optional)]
pub struct MyScript {
    // Add fields here.
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
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
        
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion {
                    delta: (dx, dy),
                },
                ..
            } => Log::info(format!("mouse motion: {}, {}", dx, dy)),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                        let pressed = event.state == ElementState::Pressed;
                        
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::KeyA) => self.move_left = pressed,
                            PhysicalKey::Code(KeyCode::KeyD) => self.move_right = pressed,
                            PhysicalKey::Code(KeyCode::KeyW) => self.move_up = pressed,
                            PhysicalKey::Code(KeyCode::KeyS) => self.move_down = pressed,
                            PhysicalKey::Code(KeyCode::Escape) => exit(0), // TODO refactor
                            _ => ()
                        }
                    },
                    WindowEvent::MouseInput { button, state, .. } => {
                        if *state == ElementState::Pressed {
                            match *button {
                                MouseButton::Left => Log::info("LMB"),
                                MouseButton::Right => Log::info("RMB"),
                                _ => (),
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        //Log::warn(format!("dt:{}, elapsed_time:{}", context.dt, context.elapsed_time));
        
        let dt = context.dt;
        
        let p = context.scene.graph[context.handle]
            .local_transform().position();
        
        let mut pos = Vector3::new(p.x, p.y, p.z);
        
        if self.move_left {
            pos.x -= dt;
        }
        if self.move_right {
            pos.x += dt;
        }
        
        if self.move_up {
            pos.y += dt;
        }
        if self.move_down {
            pos.y -= dt;
        }
        
        if pos.x != 0.0 || pos.y != 0.0 {
            context.scene.graph[context.handle].local_transform_mut().set_position(pos);
        }
        
        //context.scene.graph[context.handle].cast_mut::<Mesh>() {
            //
        //}
        //if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
    }
}
