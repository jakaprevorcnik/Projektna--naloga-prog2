use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub radij : f32,
    pub oglisca_izhodisce : Vec<Vec2>
}