use anyhow::Result;
use data::load_image;
use shrev::EventChannel;
use specs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlImageElement, WebGlRenderingContext};

use rand::Rng;

use crate::assets::Images;
use crate::components::Position;
use crate::data::Size;
use crate::gl::Image;
use crate::render_system::{DisplayEvent, RenderSystem, Sprite, Transform};
use vek::Vec2;

#[macro_use]
mod utils;
mod assets;
mod components;
mod data;
mod gl;
mod move_system;
mod render_system;
mod sprites;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame OK'");
}

#[wasm_bindgen]
pub fn start() {
    spawn_local(async { async_start().await.unwrap() })
}

pub async fn async_start() -> Result<()> {
    crate::utils::set_panic_hook();

    let frame = Rc::new(RefCell::new(None));
    let cloned_frame = frame.clone();

    let doc = window().document().expect("no global 'document' exists");

    let canvas = doc.get_element_by_id("view").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
    let mut canvas_size = get_canvas_size(&canvas);

    let context: WebGlRenderingContext = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let gl = Rc::new(context);

    let mut images = Images::new(Rc::clone(&gl));
    let id = images.load("/assets/images/bunny.png").await?;
    let image = images.image(id).unwrap();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Transform>();
    world.register::<Sprite>();
    world.insert(EventChannel::<DisplayEvent>::new());

    let component_reader = WriteStorage::<Sprite>::fetch(&world).register_reader();
    let display_reader = world
        .fetch_mut::<EventChannel<DisplayEvent>>()
        .register_reader();

    let mut rng = rand::thread_rng();

    for i in 0..2000 {
        let x: i32 = rng.gen_range(0, canvas_size.x);
        let y: i32 = rng.gen_range(0, canvas_size.y);

        world
            .create_entity()
            .with(Position::new(x as f32, y as f32))
            .with(Transform::from_image(&image))
            .with(Sprite::from_image(id))
            .build();
    }

    let mut move_system = move_system::MoveSystem;

    let mut render_system = RenderSystem::new(
        gl.clone(),
        images,
        component_reader,
        display_reader,
        canvas_size,
    );

    *cloned_frame.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let new_size = get_canvas_size(&canvas);
        if canvas_size != new_size {
            let mut chan = world.fetch_mut::<EventChannel<DisplayEvent>>();
            chan.single_write(DisplayEvent::Resized(new_size));
            canvas_size = new_size;
        }
        move_system.run_now(&mut world);
        render_system.run_now(&mut world);
        world.maintain();

        request_animation_frame(frame.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(cloned_frame.borrow().as_ref().unwrap());

    Ok(())
}

fn get_canvas_size(canvas: &web_sys::HtmlCanvasElement) -> Vec2<i32> {
    Vec2::new(canvas.width() as i32, canvas.height() as i32)
}
