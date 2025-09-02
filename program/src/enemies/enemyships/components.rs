use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyShip {
    pub x_smer : f32,
    pub bullet_shoot_timer : Timer,
    // pub oglisca_izhodisce : Vec<Vec2>,
    // pub radij: f32,
    // pub health: u32 //lahko bi dala, da imajo npr. dve življenji, ali 3.
}