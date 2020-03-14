use crate::assets::{ImageId, Images};
use crate::components::{Position, Velocity};
use crate::gl::Image;
use crate::render_system::{Sprite, Transform, WindowSize};
use rand::Rng;
use shrev::EventChannel;
use specs::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    MouseDown,
    MouseUp,
}

#[derive(Default, Debug)]
pub struct BunnyCount(pub u32);

pub struct InputSystem {
    input_reader: ReaderId<InputEvent>,
    images: Rc<Images>,
    bunny_image_id: ImageId,
    spawn: bool,
}

impl InputSystem {
    pub fn new(images: Rc<Images>, world: &mut World, bunny_image: ImageId) -> Self {
        let input_reader = world
            .fetch_mut::<EventChannel<InputEvent>>()
            .register_reader();
        InputSystem {
            images,
            input_reader,
            bunny_image_id: bunny_image,
            spawn: false,
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, WindowSize>,
        Write<'a, BunnyCount>,
        Read<'a, EventChannel<InputEvent>>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, window_size, mut bunny_count, mut input_events, updater): Self::SystemData,
    ) {
        {
            let events = input_events.read(&mut self.input_reader);
            for event in events {
                match event {
                    InputEvent::MouseDown => {
                        self.spawn = true;
                    }
                    InputEvent::MouseUp => {
                        self.spawn = false;
                    }
                }
            }
        }

        if self.spawn {
            let mut rng = rand::thread_rng();
            let size = window_size.size;
            let image = self.images.image(self.bunny_image_id).unwrap();

            let count = 20;

            for _ in 0..count {
                let bunny = entities.create();
                let angle: f32 = rng.gen_range(-std::f32::consts::PI / 2., 0.);
                let velocity: f32 = rng.gen_range(0.1, 10.);
                let x = rng.gen_range(10., 150.);
                updater.insert(bunny, Position::new(x, size.y as f32 - 10.));
                updater.insert(bunny, Velocity::from_angle(angle, velocity));
                updater.insert(bunny, Transform::from_image(image));
                updater.insert(bunny, Sprite::from_image(self.bunny_image_id));
            }
            bunny_count.0 += count;
        }
    }
}
