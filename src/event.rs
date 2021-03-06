use std::time::{Duration, SystemTime};
use std::thread::sleep;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent}
};
use crate::{
    ecs::{
        World,
        system::System,
        storage::{WriteStorage, ReadStorage},
        join::Joinable,
    },
    graphics::sprite::SpriteBatch,
    error::*,
    interface::*,
    input:: {keyboard::*, mouse::*},
    physics::*,
};

pub fn run<S: 'static>(mut interface: Interface, mut world: World, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{    
    event_loop.run(move |event, _, control_flow| {
        let interface = &mut interface;
        let world = &mut world;

        interface.process_event(&event);
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(_) => {
                    interface.graphics_context.recreate_swapchain = true;
                },
                _ => {},
            },
            Event::DeviceEvent { event, .. } => match event {
                _ => (),
            },
            Event::Resumed => {},
            Event::Suspended => {},
            Event::NewEvents(_) => {},
            Event::UserEvent(_) => {},
            Event::LoopDestroyed => {},
            Event::MainEventsCleared => { 
                interface.timer_state.tick();

                if let Err(e) = game_state.update(interface, world) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }

                if let Err(e) = game_state.draw(interface, world) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                // println!("New frame");
            },
            Event::RedrawRequested(_) => {},
            Event::RedrawEventsCleared => {},
        }
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface, world: &mut World) -> GameResult;
    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult;

    // fn mouse_button_down_event(&mut self, interface: &mut Interface, button: MouseButton, x: f32, y: f32);
    // fn mouse_button_up_event();
    // fn mouse_motion_event();
    // fn mouse_wheel_event();
}

pub struct KeyboardInputSystem {}

impl<'a> System<'a> for KeyboardInputSystem {
    type SystemData = (WriteStorage<'a, RigidBody>, ReadStorage<'a, DynamicObject>, &'a KeyboardContext);
    fn run(&mut self, (mut rigid_body, dynamic, keyboard_context): Self::SystemData) {
        let mut x = 0.0;
        let mut y = 0.0;
        
        let keys = keyboard_context.pressed_keys();

        if keys.contains(&KeyCode::W) { y -= 1.0; }
        if keys.contains(&KeyCode::A) { x -= 1.0; }
        if keys.contains(&KeyCode::S) { y += 1.0; }
        if keys.contains(&KeyCode::D) { x += 1.0; }
        
        for (rigid_body, _) in (&mut rigid_body, &dynamic).join() {
            rigid_body.desired_velocity.0 = x;
            rigid_body.desired_velocity.1 = y;
        }
    }
}