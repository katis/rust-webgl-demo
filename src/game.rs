use std::rc::Rc;

use js_sys::{Float32Array, Uint16Array};
use std::marker::PhantomData;
use vek::Vec2;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::gl::{Buffer, Gl, Image, Program, Shader};
use crate::sprites::Sprites;
use crate::utils;

static VERT: &'static str = include_str!("./quad.vert");

static FRAG: &'static str = include_str!("./quad.frag");

pub struct Game {
    gl: Rc<Gl>,
    sprites: Sprites,
    image: Image,
    program: Program,
}

impl Game {
    pub fn new(gl: Rc<Gl>, image: Image) -> Game {
        let vert = Shader::compile(gl.clone(), Gl::VERTEX_SHADER, VERT);
        let frag = Shader::compile(gl.clone(), Gl::FRAGMENT_SHADER, FRAG);

        let program = Program::compile(gl.clone(), &[vert, frag]);

        let mut sprites = Sprites::new(gl.clone());
        sprites.add(Vec2::new(0.5, 0.));
        sprites.add(Vec2::new(0.1, 0.4));

        Game {
            gl,
            sprites,
            image,
            program,
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.gl.viewport(0, 0, width, height);
    }

    pub fn draw(&mut self) {
        self.sprites.apply();

        self.gl.clear_color(0.945, 0.953, 0.957, 1.0);

        self.gl.enable(Gl::DEPTH_TEST);
        self.gl.clear(Gl::COLOR_BUFFER_BIT);

        self.program.use_program();

        let texcoord_loc = self.program.get_attrib_location("texcoord");
        let coordinates_loc = self.program.get_attrib_location("coordinates");
        let texture_uni = self.gl.get_uniform_location(&self.program, "texture");

        self.gl.active_texture(Gl::TEXTURE0);
        self.gl.bind_texture(Gl::TEXTURE_2D, Some(&self.image));
        self.gl.uniform1i(texture_uni.as_ref(), 0);

        self.sprites.draw(coordinates_loc, texcoord_loc);
    }
}
