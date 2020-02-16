use js_sys::{Float32Array, Uint16Array};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

use std::rc::Rc;

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
            self.gl
                .buffer_data_with_array_buffer_view(self.kind, &Float32Array::view(data), usage);
        }
    }

    pub fn update_u16(&self, data: &[u16], usage: u32) {
        unsafe {
            self.gl
                .buffer_data_with_array_buffer_view(self.kind, &Uint16Array::view(data), usage);
        }
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
