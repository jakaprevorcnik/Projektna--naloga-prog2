use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyShip {
    pub x_smer : f32,
    pub bullet_shoot_timer : Timer,
    // pub oglisca_izhodisce : ,
    // pub health: u32 //lahko bi dala, da imajo npr. dve življenji, ali 3.
}

#[derive(Component)]
pub struct EnemyBullet {}