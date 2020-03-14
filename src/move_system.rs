use crate::assets::Images;
use crate::components::{Position, Velocity};
use crate::render_system::{DisplayEvent, Transform, WindowSize};
use rand::Rng;
use shrev::EventChannel;
use specs::prelude::*;
use std::rc::Rc;
use vek::Vec2;

pub struct MoveSystem;
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        Read<'a, WindowSize>,
    );

    fn run(&mut self, (mut velocities, mut positions, window_size): Self::SystemData) {
        let gravity = 0.75f32;

        for (velocity, position) in (&mut velocities, &mut positions).join() {
            position.0 += velocity.0;
            velocity.0.y -= gravity;
        }

        let size: Vec2<f32> = window_size.size.numcast().unwrap();

        let mut rng = rand::thread_rng();
        for (velocity, position) in (&mut velocities, &mut positions).join() {
            if position.0.x > size.x {
                velocity.0.x *= -1.;
                position.0.x = size.x;
            } else if position.0.x < 0. {
                velocity.0.x *= -1.;
                position.0.x = 0.;
            }

            if position.0.y > size.y {
                velocity.0.y = 0.;
                position.0.y = size.y;
            } else if position.0.y < 0. {
                velocity.0.y *= -1.;
                position.0.y = 0.;

                if rng.gen_range(0., 1.) > 0.5 {
                    velocity.0.y -= rng.gen_range(0., 6.);
                }
            }
        }
    }
}
