use crate::render_system::Transform;
use specs::prelude::*;

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = WriteStorage<'a, Transform>;

    fn run(&mut self, mut transforms: Self::SystemData) {
        for transform in (&mut transforms).join() {
            transform.rotate(0.1);
        }
    }
}
