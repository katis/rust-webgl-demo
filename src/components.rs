use specs::prelude::*;
use specs::{Component, DenseVecStorage};
use vek::Vec2;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(DenseVecStorage)]
pub struct Position(Vec2<f32>);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Vec2::new(x, y))
    }

    pub fn vector(&self) -> Vec2<f32> {
        self.0
    }
}
