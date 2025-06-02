use crate::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}
