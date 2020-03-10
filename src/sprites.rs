use specs::prelude::*;
use std::mem;
use std::rc::Rc;
use vek::Vec2;

use crate::gl::{BoundBuffer, Buffer, Gl, Image};

type Radians = f32;

pub struct Sprites {
    gl: Rc<Gl>,

    needs_resize: bool,

    /*
    positions: Vec<Vec2<f32>>,
    transforms: Vec<Transform>,
    */
    uv_buffer: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    texcoords_buffer: Buffer,
}

impl Sprites {
    pub fn new(gl: Rc<Gl>) -> Sprites {
        Sprites {
            gl: gl.clone(),
            needs_resize: false,
            /*
            positions: Vec::new(),
            transforms: Vec::new(),
            */
            uv_buffer: Buffer::new(gl.clone(), Gl::ARRAY_BUFFER),
            vertex_buffer: Buffer::new(gl.clone(), Gl::ARRAY_BUFFER),
            index_buffer: Buffer::new(gl.clone(), Gl::ARRAY_BUFFER),
            texcoords_buffer: Buffer::new(gl.clone(), Gl::ARRAY_BUFFER),
        }
    }

    /*
    pub fn rotate(&mut self, sprite: Sprite, angle: Radians) {
        let mut transform = &mut self.transforms[sprite as usize];

        transform.up.rotate_z(angle);
        transform.right.rotate_z(angle);
    }

    pub fn translate(&mut self, sprite: Sprite, delta: Vec2<f32>) {
        let mut position = &mut self.positions[sprite as usize];

        *position = *position + delta;
    }
    */

    fn resize_buffers(&mut self) {
        if !self.needs_resize {
            return;
        }
        self.needs_resize = false;

        let vertex_length = self.len() * mem::size_of::<Vec2<[f32; 4]>>();
        let vertex_buffer = Buffer::new_sized(
            self.gl.clone(),
            Gl::ARRAY_BUFFER,
            vertex_length as i32,
            Gl::DYNAMIC_DRAW,
        );

        let index_length = self.len() * mem::size_of::<[u16; 6]>();
        let index_buffer = Buffer::new_sized(
            self.gl.clone(),
            Gl::ELEMENT_ARRAY_BUFFER,
            index_length as i32,
            Gl::DYNAMIC_DRAW,
        );

        let texcoords_length = self.len() * mem::size_of::<[Vec<f32>; 4]>();
        let texcoords_buffer = Buffer::new_sized(
            self.gl.clone(),
            Gl::ARRAY_BUFFER,
            texcoords_length as i32,
            Gl::DYNAMIC_DRAW,
        );

        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
        self.texcoords_buffer = texcoords_buffer;
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn apply(&mut self) {
        self.resize_buffers();

        let len = self.len();

        let Sprites {
            ref mut vertex_buffer,
            ref mut index_buffer,
            ref mut texcoords_buffer,
            ..
        } = self;

        /*
        for (i, (t, p)) in transforms.iter_mut().zip(positions.iter_mut()).enumerate() {
            let verts = t.vertices(p);

            let offset = i * mem::size_of_val(&verts);

            vertices.update_sub_vec2(&verts, offset as u32);
        }
        */

        let indices = index_buffer.bind();
        for i in 0..len {
            let n = (i * 4) as u16;
            let indexes = [n, n + 2, n + 1, n, n + 3, n + 2];
            let offset = i * mem::size_of::<[u16; 6]>();
            indices.update_sub_u16(&indexes, offset as u32)
        }

        let texcoords = texcoords_buffer.bind();
        let coords = [
            Vec2::new(0.0f32, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
        ];
        for i in 0..len {
            let offset = i * mem::size_of_val(&coords);
            texcoords.update_sub_vec2(&coords, offset as u32)
        }
    }

    pub fn draw(&self, coordinates_attr: u32, texcoord_attr: u32) {
        self.texcoords_buffer.bind();
        self.gl
            .vertex_attrib_pointer_with_i32(texcoord_attr, 2, Gl::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(texcoord_attr);

        self.vertex_buffer.bind();
        self.index_buffer.bind();

        self.gl
            .vertex_attrib_pointer_with_i32(coordinates_attr, 2, Gl::FLOAT, false, 0, 0);

        self.gl.enable_vertex_attrib_array(coordinates_attr);

        let count = (self.len() * 6) as i32;

        self.gl
            .draw_elements_with_i32(Gl::TRIANGLES, count, Gl::UNSIGNED_SHORT, 0);
    }
}
