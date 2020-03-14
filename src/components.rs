use specs::prelude::*;
use specs::{Component, DenseVecStorage};
use vek::Vec2;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(DenseVecStorage)]
pub struct Position(pub Vec2<f32>);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Vec2::new(x, y))
    }

    pub fn vector(&self) -> Vec2<f32> {
        self.0
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(DenseVecStorage)]
pub struct Velocity(pub Vec2<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vec2::new(x, y))
    }

    pub fn from_angle(angle_radians: f32, velocity: f32) -> Self {
        let mut vel = Velocity(Vec2::unit_x());
        vel.0.rotate_z(angle_radians);
        vel.0 *= velocity;
        vel
    }
}
