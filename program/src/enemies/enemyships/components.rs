use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyShip {
    pub x_smer : f32,
    pub bullet_shoot_timer : Timer,
}