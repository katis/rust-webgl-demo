use js_sys::{Float32Array, Uint16Array};
use std::mem;
use std::slice;
use vek::{Vec2, Vec3};
use web_sys::{
    HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
};

use std::rc::Rc;
use wasm_bindgen::__rt::core::ops::Deref;

pub type Gl = WebGlRenderingContext;

// Buffer

pub struct Buffer {
    gl: Rc<Gl>,
    kind: u32,
    buffer: WebGlBuffer,
}

impl Buffer {
    pub fn new(gl: Rc<Gl>, kind: u32) -> Buffer {
        let buffer = gl.create_buffer().expect("should create buffer");
        Buffer { gl, kind, buffer }
    }

    pub fn new_sized(gl: Rc<Gl>, kind: u32, size: i32, usage: u32) -> Buffer {
        let buffer = Buffer::new(gl, kind);

        buffer
            .gl
            .bind_buffer(buffer.kind, Some(buffer.buffer.as_ref()));
        buffer.gl.buffer_data_with_i32(buffer.kind, size, usage);

        buffer
    }

    pub fn bind(&self) -> BoundBuffer {
        self.gl.bind_buffer(self.kind, Some(self.buffer.as_ref()));

        BoundBuffer {
            gl: self.gl.as_ref(),
            kind: self.kind,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(&self.buffer));
    }
}

pub struct BoundBuffer<'a> {
    gl: &'a Gl,
    kind: u32,
}

impl<'a> BoundBuffer<'a> {
    pub fn update_f32(&self, data: &[f32], usage: u32) {
        unsafe {
            self.update(data, usage);
        }
    }

    pub fn update_vec2(&self, vectors: &[Vec2<f32>], usage: u32) {
        unsafe {
            self.update(vectors, usage);
        }
    }

    pub fn update_vec3(&self, vectors: &[Vec3<f32>], usage: u32) {
        unsafe {
            self.update(vectors, usage);
        }
    }

    pub fn update_sub_vec2(&self, vectors: &[Vec2<f32>], offset: u32) {
        unsafe {
            self.update_sub(vectors, offset);
        }
    }

    pub fn update_u16(&self, data: &[u16], usage: u32) {
        unsafe {
            self.update(data, usage);
        }
    }

    pub fn update_sub_u16(&self, vectors: &[u16], offset: u32) {
        unsafe {
            self.update_sub(vectors, offset);
        }
    }

    unsafe fn update<T>(&self, data: &[T], usage: u32) {
        let raw_data =
            slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * mem::size_of::<T>());
        self.gl
            .buffer_data_with_u8_array(self.kind, raw_data, usage);
    }

    unsafe fn update_sub<T>(&self, data: &[T], offset: u32) {
        let raw_data =
            slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * mem::size_of::<T>());
        self.gl
            .buffer_sub_data_with_i32_and_u8_array(self.kind, offset as i32, raw_data);
    }
}

// Program

pub struct Program {
    gl: Rc<Gl>,
    program: WebGlProgram,
}

impl Program {
    pub fn compile(gl: Rc<Gl>, shaders: &[Shader]) -> Program {
        let program = gl.create_program().expect("should create program");

        for shader in shaders.iter() {
            gl.attach_shader(&program, &shader);
        }

        gl.link_program(&program);

        Program { gl, program }
    }

    pub fn use_program(&self) {
        self.gl.use_program(Some(&self.program));
    }

    pub fn get_attrib_location(&self, name: &str) -> u32 {
        self.gl.get_attrib_location(&self.program, name) as u32
    }
}

impl std::ops::Deref for Program {
    type Target = WebGlProgram;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}

// Shader

pub struct Shader {
    gl: Rc<Gl>,
    shader: WebGlShader,
}

impl Shader {
    pub fn compile(gl: Rc<Gl>, kind: u32, source: &str) -> Shader {
        let shader = gl.create_shader(kind).expect("should create shader");

        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        Shader { gl, shader }
    }
}

impl std::ops::Deref for Shader {
    type Target = WebGlShader;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.shader));
    }
}

// Image

pub struct Image {
    gl: Rc<Gl>,
    texture: WebGlTexture,
}

impl Image {
    pub fn from_image_element(gl: Rc<Gl>, element: &HtmlImageElement) -> Image {
        let texture = gl.create_texture().expect("should create texture");
        gl.bind_texture(Gl::TEXTURE_2D, Some(&texture));

        gl.tex_image_2d_with_u32_and_u32_and_image(
            Gl::TEXTURE_2D,
            0,
            Gl::RGBA as i32,
            Gl::RGBA,
            Gl::UNSIGNED_BYTE,
            element,
        )
        .unwrap();

        gl.generate_mipmap(Gl::TEXTURE_2D);

        gl.bind_texture(Gl::TEXTURE_2D, None);

        Image { gl, texture }
    }
}

impl std::ops::Deref for Image {
    type Target = WebGlTexture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}
