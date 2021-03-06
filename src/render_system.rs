use crate::assets::{ImageId, Images};
use crate::components::Position;
use crate::gl::{Buffer, Gl, Image, Program, Shader, TypedBuffer};
use shrev::EventChannel;
use specs::prelude::*;
use specs::{SystemData, WriteStorage};
use std::rc::Rc;
use vek::column_major::Mat4;
use vek::{FrustumPlanes, Vec2};
use web_sys::WebGlUniformLocation;

static VERT: &'static str = include_str!("./quad.vert");

static FRAG: &'static str = include_str!("./quad.frag");

#[derive(Clone, Copy, Debug)]
pub enum DisplayEvent {
    Resized(Vec2<i32>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowSize {
    pub size: Vec2<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sprite {
    batch_id: u32,
}

impl Sprite {
    pub fn from_image(image_id: ImageId) -> Self {
        Sprite {
            batch_id: image_id.id,
        }
    }
}

impl Component for Sprite {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    up: Vec2<f32>,
    right: Vec2<f32>,
}

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}

impl Transform {
    pub fn new(up: Vec2<f32>, right: Vec2<f32>) -> Self {
        Transform { up, right }
    }

    pub fn from_image(image: &Image) -> Self {
        Transform::new(
            Vec2::new(0., image.height as f32 / 2.),
            Vec2::new(image.width as f32 / 2., 0.),
        )
    }

    pub fn rotate(&mut self, radians: f32) {
        self.up.rotate_z(radians);
        self.right.rotate_z(radians);
    }
}

#[repr(C)]
#[derive(Debug)]
struct Vertex {
    vertex: Vec2<f32>,
    texcoord: Vec2<u16>,
}

#[repr(C)]
#[derive(Debug)]
struct VertexData([Vertex; 4]);

impl VertexData {
    fn new(position: &Vec2<f32>, Transform { up, right }: &Transform) -> Self {
        let v1 = position - right + up;
        let v2 = position + right + up;
        let v3 = position - right - up;
        let v4 = position + right - up;
        VertexData([
            Vertex {
                vertex: v1,
                texcoord: Vec2::new(0, 0),
            },
            Vertex {
                vertex: v2,
                texcoord: Vec2::new(std::u16::MAX, 0),
            },
            Vertex {
                vertex: v3,
                texcoord: Vec2::new(0, std::u16::MAX),
            },
            Vertex {
                vertex: v4,
                texcoord: Vec2::new(std::u16::MAX, std::u16::MAX),
            },
        ])
    }
}

pub struct RenderSystem {
    gl: Rc<Gl>,

    component_reader: ReaderId<ComponentEvent>,

    batches: Vec<SpriteBatch>,

    camera: Mat4<f32>,
    program: Program,
    projection_uni: WebGlUniformLocation,
    texture_uni: WebGlUniformLocation,
    window_size: Vec2<i32>,
}

impl RenderSystem {
    pub fn new(gl: Rc<Gl>, images: &Images, world: &World, canvas_size: Vec2<i32>) -> Self {
        let component_reader = WriteStorage::<Sprite>::fetch(&world).register_reader();

        let program = {
            let vert = Shader::compile(gl.clone(), Gl::VERTEX_SHADER, VERT);
            let frag = Shader::compile(gl.clone(), Gl::FRAGMENT_SHADER, FRAG);

            Program::compile(gl.clone(), &[vert, frag])
        };

        let projection_uni = program.get_uniform_location("projection").unwrap();
        let texture_uni = program.get_uniform_location("texture").unwrap();
        let texcoord_attr = program.get_attrib_location("texcoord");
        let coordinates_attr = program.get_attrib_location("coordinates");

        let mut batches = Vec::new();
        for (_, image) in images.images() {
            let batch =
                SpriteBatch::new(gl.clone(), image.clone(), coordinates_attr, texcoord_attr);
            batches.push(batch);
        }

        let camera = camera_mat(canvas_size);

        RenderSystem {
            gl,
            component_reader,

            batches,
            camera,
            program,
            projection_uni,
            texture_uni,
            window_size: canvas_size,
        }
    }
}

fn camera_mat(size: Vec2<i32>) -> Mat4<f32> {
    Mat4::orthographic_lh_zo(FrustumPlanes {
        left: 0.0f32,
        right: size.x as f32,
        bottom: 0.,
        top: size.y as f32,
        near: 1.,
        far: -1.,
    })
}

impl RenderSystem {
    pub fn resize(&mut self, size: Vec2<i32>) {
        if self.window_size == size {
            return;
        }
        self.window_size = size;

        self.gl.viewport(0, 0, size.x, size.y);
        self.camera = camera_mat(size);
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Position>,
        Read<'a, WindowSize>,
    );

    fn run(
        &mut self,
        (entities, mut sprites, transforms, positions, mut window_size): Self::SystemData,
    ) {
        {
            let events = sprites.channel().read(&mut self.component_reader);
            for event in events {
                match event {
                    ComponentEvent::Inserted(id) => {
                        let entity = entities.entity(*id);
                        let sprite = sprites.get(entity).unwrap();
                        let mut batch = &mut self.batches[sprite.batch_id as usize];
                        batch.add(*id);
                    }
                    ComponentEvent::Removed(id) => {
                        let entity = entities.entity(*id);
                        let sprite = sprites.get(entity).unwrap();
                        let mut batch = &mut self.batches[sprite.batch_id as usize];
                        batch.remove(*id);
                    }
                    _ => {}
                }
            }
        }

        self.resize(window_size.size);

        self.gl.disable(Gl::DEPTH_TEST);
        self.gl.enable(Gl::BLEND);
        self.gl.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);
        self.gl.clear(Gl::COLOR_BUFFER_BIT);

        self.program.use_program();

        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&self.projection_uni),
            false,
            self.camera.as_col_slice(),
        );

        for batch in self.batches.iter_mut() {
            batch.render(&self.texture_uni, &transforms, &positions);
        }
    }
}

struct SpriteBatch {
    gl: Rc<Gl>,
    image: Image,
    len: i32,
    new_len: i32,

    coordinates_attr: u32,
    texcoord_attr: u32,

    entities: BitSet,

    vertex_data_buffer: TypedBuffer<VertexData>,
    index_buffer: TypedBuffer<[u16; 6]>,

    vertex_data: Vec<VertexData>,
    index_data: Vec<[u16; 6]>,
}

impl SpriteBatch {
    pub fn new(gl: Rc<Gl>, image: Image, coordinates_attr: u32, texcoord_attr: u32) -> Self {
        SpriteBatch {
            gl: gl.clone(),
            image,
            len: -1,
            new_len: 0,

            coordinates_attr,
            texcoord_attr,

            entities: BitSet::new(),

            vertex_data_buffer: TypedBuffer::new(gl.clone(), Gl::ARRAY_BUFFER, Gl::DYNAMIC_DRAW, 0),
            index_buffer: TypedBuffer::new(
                gl.clone(),
                Gl::ELEMENT_ARRAY_BUFFER,
                Gl::DYNAMIC_DRAW,
                0,
            ),
            vertex_data: Vec::new(),
            index_data: Vec::new(),
        }
    }

    pub fn render(
        &mut self,
        texture_uni: &WebGlUniformLocation,
        transforms: &ReadStorage<Transform>,
        positions: &ReadStorage<Position>,
    ) {
        self.resize_buffers();

        let mut vertexdata = self.vertex_data_buffer.bind();
        self.vertex_data.clear();
        self.vertex_data.reserve(self.len as usize);

        let vertices = (&self.entities, transforms, positions)
            .join()
            .map(|(_, transform, position)| VertexData::new(&position.vector(), transform));

        self.vertex_data.extend(vertices);

        vertexdata.update_sub(&self.vertex_data, 0);

        self.draw(texture_uni);
    }

    fn resize_buffers(&mut self) {
        if self.len >= self.new_len {
            return;
        }
        self.len = self.new_len;

        self.vertex_data_buffer.resize(self.len);
        self.index_buffer.resize(self.len);

        self.index_data.clear();
        self.index_data.reserve(self.len as usize);

        let iter = (0..self.len).map(|i| {
            let n = (i * 4) as u16;
            [n, n + 1, n + 2, n + 2, n + 3, n + 1]
        });
        self.index_data.extend(iter);

        let mut indices = self.index_buffer.bind();
        indices.update_sub(&self.index_data, 0);
    }

    fn add(&mut self, entity: u32) {
        if !self.entities.add(entity) {
            self.new_len += 1;
        }
    }

    fn remove(&mut self, entity: u32) {
        if self.entities.remove(entity) {
            self.new_len -= 1;
        }
    }

    fn draw(&mut self, texture_uni: &WebGlUniformLocation) {
        self.gl.active_texture(Gl::TEXTURE0);
        self.gl.bind_texture(Gl::TEXTURE_2D, Some(&self.image));
        self.gl.uniform1i(Some(texture_uni.as_ref()), 0);

        self.vertex_data_buffer.bind();

        self.gl.vertex_attrib_pointer_with_i32(
            self.coordinates_attr,
            2,
            Gl::FLOAT,
            false,
            std::mem::size_of::<Vertex>() as i32,
            0,
        );
        self.gl.enable_vertex_attrib_array(self.coordinates_attr);

        self.gl.vertex_attrib_pointer_with_i32(
            self.texcoord_attr,
            2,
            Gl::UNSIGNED_SHORT,
            true,
            std::mem::size_of::<Vertex>() as i32,
            std::mem::size_of::<Vec2<f32>>() as i32,
        );
        self.gl.enable_vertex_attrib_array(self.texcoord_attr);

        self.index_buffer.bind();

        self.gl
            .draw_elements_with_i32(Gl::TRIANGLES, self.len * 6, Gl::UNSIGNED_SHORT, 0);
    }
}
