use js_sys::{Function, JsString, Promise};
use std::mem::MaybeUninit;
use std::rc::Rc;
use web_sys::HtmlImageElement;

use wasm_bindgen_futures::future_to_promise;

use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use wasm_bindgen::__rt::core::fmt::{Error, Formatter};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

#[derive(Debug, PartialEq, Eq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size { width, height }
    }
}

#[wasm_bindgen(module = "/js/load_image.js")]
extern "C" {
    fn raw_load_image(url: &str) -> Promise;
}

pub async fn load_image(url: &str) -> Result<HtmlImageElement, JsError> {
    let promise = raw_load_image(url);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    match result {
        Ok(el) => Ok(el.unchecked_into::<HtmlImageElement>()),
        Err(err) => Err(err.into()),
    }
}

pub struct JsError {
    name: String,
    message: String,
}

impl std::convert::From<JsValue> for JsError {
    fn from(value: JsValue) -> Self {
        if value.has_type::<js_sys::Error>() {
            let error: js_sys::Error = value.unchecked_into();
            JsError {
                name: to_string(&error.name()),
                message: to_string(&error.message()),
            }
        } else {
            JsError {
                name: "UnknownError".into(),
                message: format!("{:?}", &value),
            }
        }
    }
}

impl std::error::Error for JsError {}

impl std::fmt::Display for JsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.name, &self.message)
    }
}

impl std::fmt::Debug for JsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.name, &self.message)
    }
}

fn to_string(js_string: &JsString) -> String {
    decode_utf16(js_string.iter())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect::<String>()
}
