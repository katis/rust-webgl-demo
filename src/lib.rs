#![feature(generators, generator_trait)]

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlImageElement, WebGlRenderingContext};

use crate::data::Size;
use crate::gl::Image;

#[macro_use]
mod utils;
mod data;
mod game;
mod gl;
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
pub fn start(img: HtmlImageElement) {
    crate::utils::set_panic_hook();

    let frame = Rc::new(RefCell::new(None));
    let cloned_frame = frame.clone();

    let doc = window().document().expect("no global 'document' exists");

    let canvas = doc.get_element_by_id("view").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    let context: WebGlRenderingContext = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let gl = Rc::new(context);

    let image = Image::from_image_element(gl.clone(), &img);

    let mut game = crate::game::Game::new(gl.clone(), image);

    let mut canvas_size = get_canvas_size(&canvas);

    *cloned_frame.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let new_size = get_canvas_size(&canvas);
        if canvas_size != new_size {
            game.resize(new_size.width, new_size.height);
            canvas_size = new_size;
        }
        game.draw();
        request_animation_frame(frame.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(cloned_frame.borrow().as_ref().unwrap());
}

fn get_canvas_size(canvas: &web_sys::HtmlCanvasElement) -> Size<i32> {
    Size::new(canvas.width() as i32, canvas.height() as i32)
}
