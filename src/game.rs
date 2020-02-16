use std::rc::Rc;

use js_sys::{Float32Array, Uint16Array};
use std::marker::PhantomData;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::gl::{Buffer, Gl, Program, Shader};
use crate::utils;

static VERT: &'static str = include_str!("./quad.vert");

static FRAG: &'static str = include_str!("./quad.frag");

pub struct Game {
    gl: Rc<Gl>,
    program: Program,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl Game {
    pub fn new(gl: Rc<Gl>) -> Game {
        let vert = Shader::compile(gl.clone(), Gl::VERTEX_SHADER, VERT);
        let frag = Shader::compile(gl.clone(), Gl::FRAGMENT_SHADER, FRAG);

        let program = Program::compile(gl.clone(), &[vert, frag]);

        let vertices: [f32; 12] = [
            -0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0,
        ];

        let indices = [3, 2, 1, 3, 1, 0];

        let vertex_buffer = Buffer::new(gl.clone(), Gl::ARRAY_BUFFER);
        vertex_buffer.bind().update_f32(&vertices, Gl::STATIC_DRAW);

        let index_buffer = Buffer::new(gl.clone(), Gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.bind().update_u16(&indices, Gl::STATIC_DRAW);

        Game {
            gl,
            program,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.gl.viewport(0, 0, width, height);
    }

    pub fn draw(&mut self) {
        self.gl.clear_color(0.945, 0.953, 0.957, 1.0);

        self.gl.enable(Gl::DEPTH_TEST);
        self.gl.clear(Gl::COLOR_BUFFER_BIT);

        self.program.use_program();
        self.vertex_buffer.bind();
        self.index_buffer.bind();

        let coordinates_loc = self.program.get_attrib_location("coordinates");

        self.gl
            .vertex_attrib_pointer_with_i32(coordinates_loc, 3, Gl::FLOAT, false, 0, 0);

        self.gl.enable_vertex_attrib_array(coordinates_loc);

        self.gl
            .draw_elements_with_i32(Gl::TRIANGLES, 6, Gl::UNSIGNED_SHORT, 0);
    }
}
