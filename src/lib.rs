use anyhow::Result;
use data::load_image;
use shrev::EventChannel;
use specs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlElement, HtmlImageElement, WebGlRenderingContext};

use rand::Rng;

use crate::assets::Images;
use crate::components::{Position, Velocity};
use crate::data::Size;
use crate::game::Game;
use crate::gl::Image;
use crate::input_system::{InputEvent, InputSystem};
use crate::render_system::{DisplayEvent, RenderSystem, Sprite, Transform, WindowSize};
use std::sync::Mutex;
use vek::Vec2;

#[macro_use]
mod utils;
mod assets;
mod components;
mod data;
mod game;
mod gl;
mod input_system;
mod move_system;
mod render_system;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame OK'");
}

#[wasm_bindgen]
pub fn start(counter_el: HtmlElement) {
    spawn_local(async { async_start(counter_el).await.unwrap() })
}

pub async fn async_start(counter_el: HtmlElement) -> Result<()> {
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

    let win = window();
    let mut game = Game::init(Rc::new(context), &win, canvas_size).await?;

    *cloned_frame.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let new_size = get_canvas_size(&canvas);

        game.process_events();
        game.run_world(new_size);

        counter_el.set_inner_text(&format!("Bunnies: {}", game.bunny_count()));

        request_animation_frame(frame.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(cloned_frame.borrow().as_ref().unwrap());

    Ok(())
}

fn get_canvas_size(canvas: &web_sys::HtmlCanvasElement) -> Vec2<i32> {
    Vec2::new(canvas.width() as i32, canvas.height() as i32)
}
