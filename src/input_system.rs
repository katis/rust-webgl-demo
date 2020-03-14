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

pub struct InputSystem {
    input_reader: ReaderId<InputEvent>,
    images: Rc<Images>,
    bunny_image_id: ImageId,
    spawn: bool,
}

impl InputSystem {
    pub fn new(
        images: Rc<Images>,
        input_reader: ReaderId<InputEvent>,
        bunny_image: ImageId,
    ) -> Self {
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
        Read<'a, EventChannel<InputEvent>>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, window_size, mut input_events, updater): Self::SystemData) {
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
            let bunny = entities.create();
            updater.insert(bunny, Position::new(10., size.y as f32 - 10.));
            updater.insert(
                bunny,
                Velocity::from_angle(rng.gen_range(-std::f32::consts::PI, 0.), 4.),
            );
            updater.insert(bunny, Transform::from_image(image));
            updater.insert(bunny, Sprite::from_image(self.bunny_image_id));
        }
    }
}
