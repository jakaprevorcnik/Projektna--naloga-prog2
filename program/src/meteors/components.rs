use bevy::prelude::*;

#[derive(Component)]
pub struct Meteor {
    pub radij : f32,
    pub oglisca_izhodisce : [Vec2; 10],
    pub kot : f32,
    pub oglisca_pozicija : [Vec2; 10],
}
