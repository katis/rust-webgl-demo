use crate::assets::{ImageId, Images};
use crate::components::{Position, Velocity};
use crate::gl::Gl;
use crate::input_system::{BunnyCount, InputEvent, InputSystem};
use crate::move_system::MoveSystem;
use crate::render_system::{DisplayEvent, RenderSystem, Sprite, Transform, WindowSize};
use anyhow::Result;
use rand::Rng;
use specs::prelude::*;
use specs::shrev::EventChannel;
use std::rc::Rc;
use std::sync::Mutex;
use vek::Vec2;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Window;

pub struct Game {
    gl: Rc<Gl>,
    images: Rc<Images>,
    dom_event_handlers: DomEvents,
    world: World,
    input_system: InputSystem,
    move_system: MoveSystem,
    render_system: RenderSystem,
}

impl Game {
    pub async fn init(gl: Rc<Gl>, window: &Window, canvas_size: Vec2<i32>) -> Result<Self> {
        let images = load_images(gl.clone()).await?;
        let images = Rc::new(images);
        let bunny_image = images.find_image_id("/assets/images/bunny.png").unwrap();
        let mut world = init_world(canvas_size, &images);
        Ok(Game {
            gl: gl.clone(),
            images: Rc::new(Images::new(gl.clone())),
            dom_event_handlers: DomEvents::register(window),
            input_system: InputSystem::new(images.clone(), &mut world, bunny_image),
            render_system: RenderSystem::new(gl.clone(), &images, &world, canvas_size),
            move_system: MoveSystem,
            world,
        })
    }

    pub fn process_events(&mut self) {
        let mut channel = self.world.fetch_mut::<EventChannel<InputEvent>>();
        self.dom_event_handlers.process_input(&mut channel);
    }

    pub fn bunny_count(&self) -> u32 {
        self.world.fetch::<BunnyCount>().0
    }

    pub fn run_world(&mut self, canvas_size: Vec2<i32>) {
        {
            let mut window_size = self.world.fetch_mut::<WindowSize>();
            window_size.size = canvas_size;
        }

        self.input_system.run_now(&mut self.world);
        self.move_system.run_now(&mut self.world);
        self.render_system.run_now(&mut self.world);
        self.world.maintain();
    }
}

async fn load_images(gl: Rc<Gl>) -> Result<Images> {
    let mut images = Images::new(gl);
    images.load("/assets/images/bunny.png").await?;
    Ok(images)
}

fn init_world(canvas_size: Vec2<i32>, images: &Images) -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Transform>();
    world.register::<Sprite>();
    world.insert(EventChannel::<DisplayEvent>::new());
    world.insert(EventChannel::<InputEvent>::new());
    world.insert(WindowSize { size: canvas_size });
    world.insert(BunnyCount(3));

    let mut rng = rand::thread_rng();

    let bunny_id = images.find_image_id("/assets/images/bunny.png").unwrap();
    let bunny_image = images.image(bunny_id).unwrap();

    for _ in 0..3 {
        let x: i32 = rng.gen_range(0, canvas_size.x);
        let y: i32 = rng.gen_range(0, canvas_size.y);

        world
            .create_entity()
            .with(Position::new(x as f32, y as f32))
            .with(Velocity::from_angle(
                rng.gen_range(-std::f32::consts::PI, std::f32::consts::PI),
                4.,
            ))
            .with(Transform::from_image(&bunny_image))
            .with(Sprite::from_image(bunny_id))
            .build();
    }

    world
}

struct DomEvents {
    mouse_down: Closure<dyn Fn()>,
    mouse_up: Closure<dyn Fn()>,
    input_events: Rc<Mutex<Vec<InputEvent>>>,
}

impl DomEvents {
    pub fn register(window: &Window) -> Self {
        let input_events = Rc::new(Mutex::new(Vec::<InputEvent>::new()));
        let events = input_events.clone();
        let mouse_down = Closure::wrap(Box::new(move || {
            let mut evs = events.lock().unwrap();
            evs.push(InputEvent::MouseDown);
        }) as Box<dyn Fn()>);
        window
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())
            .unwrap();
        let events = input_events.clone();
        let mouse_up = Closure::wrap(Box::new(move || {
            let mut evs = events.lock().unwrap();
            evs.push(InputEvent::MouseUp);
        }) as Box<dyn Fn()>);
        window
            .add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())
            .unwrap();

        DomEvents {
            mouse_down,
            mouse_up,
            input_events,
        }
    }

    pub fn process_input(&self, channel: &mut EventChannel<InputEvent>) {
        let mut events = self.input_events.lock().unwrap();
        channel.drain_vec_write(&mut events);
    }

    pub fn clear(&mut self) {
        self.input_events.lock().unwrap().clear();
    }
}
