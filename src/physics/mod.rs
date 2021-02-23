use crate::{
    ecs::{
        component::Component,
        system::System,
        storage::{ 
            NullStorage,
            VecStorage,
            WriteStorage,
            ReadStorage,
        },
        join::Joinable,
    },
    graphics::{
        sprite::Sprite,
    }
};

use std::time::Duration;

#[derive(Default)]
pub struct RigidBody {
    pub(crate) velocity: (f32, f32),
    pub(crate) previous_velocity: (f32, f32),
    pub(crate) transition_speed: (f32, f32),
    pub(crate) desired_velocity: (f32, f32),
}

impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Position {
    pub previous_position: (f32, f32),
    pub current_position: (f32, f32),
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct DynamicObject {}

impl Component for DynamicObject {
    type Storage = NullStorage<Self>;
}

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, RigidBody>, Duration);

    fn run(&mut self, (mut rigid_body, delta_time): Self::SystemData) {
        for rigid_body in (&mut rigid_body).join() {
            rigid_body.previous_velocity = rigid_body.velocity;
            let mut velocity: (f32, f32) = (0.0, 0.0);
            velocity.0 = rigid_body.velocity.0 * 
                    (1.0 - delta_time.as_secs_f32() * rigid_body.transition_speed.0) + 
                    rigid_body.desired_velocity.0 * (delta_time.as_secs_f32() * rigid_body.transition_speed.0);
            
            velocity.1 = rigid_body.velocity.1 * 
                    (1.0 - delta_time.as_secs_f32() * rigid_body.transition_speed.1) + 
                    rigid_body.desired_velocity.1 * (delta_time.as_secs_f32() * rigid_body.transition_speed.1);

            if (velocity.0 < 0.005 && velocity.0 > -0.005) && rigid_body.desired_velocity.0 == 0.0 { velocity.0 = 0.0}
            if (velocity.1 < 0.005 && velocity.1 > -0.005) && rigid_body.desired_velocity.1 == 0.0 { velocity.1 = 0.0}

            // println!("{:?}", delta_time);
            if delta_time.as_secs_f64() < 0.010 {
                println!("[ERROR]: delta_time significantly LOWER than expexted.");
            }
            if delta_time.as_secs_f64() > 0.020 {
                println!("[ERROR]: delta_time significantly HIGHER than expexted.");
            }
            
            rigid_body.velocity = velocity;
        }
    }
}

pub struct PositionSystem {}

impl<'a> System<'a> for PositionSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, RigidBody>, f32);

    fn run(&mut self, (mut pos, rigid_body, alpha): Self::SystemData) {
        for (pos, rigid_body) in (&mut pos, &rigid_body).join() {
            pos.previous_position = pos.current_position;

            pos.current_position.0 += (rigid_body.velocity.0 + rigid_body.previous_velocity.0)/2.0;
            pos.current_position.1 += (rigid_body.velocity.1 + rigid_body.previous_velocity.1)/2.0;

            pos.current_position.0 = pos.current_position.0 * alpha + pos.previous_position.0 * (1.0 - alpha); 
            pos.current_position.1 = pos.current_position.1 * alpha + pos.previous_position.1 * (1.0 - alpha); 
        }
    }
}

pub struct SpriteMove {}

impl<'a> System<'a> for SpriteMove {
    type SystemData = (WriteStorage<'a, Sprite>, ReadStorage<'a, Position>);

    fn run(&mut self, (mut sprite, pos): Self::SystemData) {
        for (sprite, pos) in (&mut sprite, &pos).join() {
            sprite.update_rect([pos.current_position.0 as f32, pos.current_position.1 as f32]);
        }
    }
}