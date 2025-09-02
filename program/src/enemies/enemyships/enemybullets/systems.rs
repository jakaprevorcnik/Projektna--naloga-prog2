use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::enemies::systems::create_vertex_vector;
use crate::enemies::components::Enemy;
use crate::enemies::enemyships::enemybullets::components::EnemyBullet;
use crate::enemies::enemyships::components::EnemyShip;

const ENEMYBULLET_SHOOT_TIME: f32 = 1.8;
const ENEMYBULLET_SPEED: f32 = 200.0;

const ENEMYBULLET_RADIUS: f32 = 18.0;
const OGLISCA_ENEMYBULLET_SLIKA: [Vec2; 12] = [Vec2::new(0., 7.), Vec2::new(1., 4.), Vec2::new(4., 0.),
    Vec2::new(6., 0.), Vec2::new(9., 4.), Vec2::new(10., 7.), Vec2::new(10., 24.), Vec2::new(9., 34.),
    Vec2::new(7., 35.), Vec2::new(3., 35.), Vec2::new(1., 34.), Vec2::new(0., 24.)];
const CENTER_ENEMYBULLET_SLIKA: Vec2 = Vec2::new(5.5, 17.5);


pub fn tick_enemybullet_timer(
    mut enemyship_query: Query<&mut EnemyShip>,
    time: Res<Time>
) {
    for mut enemy in enemyship_query.iter_mut() {
        enemy.bullet_shoot_timer.tick(time.delta());
    }
}


pub fn shoot_enemy_bullet(
    enemyship_query: Query<(&EnemyShip, &Transform)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
){
    for (enemyship, enemyship_transform) in enemyship_query.iter() {
        if enemyship.bullet_shoot_timer.finished() {

            let oglisca= create_vertex_vector(
                OGLISCA_ENEMYBULLET_SLIKA.to_vec(), CENTER_ENEMYBULLET_SLIKA, (180_f32).to_radians(), 1.0);

            commands.spawn((
                Sprite {
                    image: asset_server.load("sprites/spaceMissiles_037.png"),
                            ..Default::default()
                },
                Transform::from_xyz(
                    enemyship_transform.translation.x,
                    enemyship_transform.translation.y - 40.0,
                    0.0,
                    ).with_rotation(Quat::from_rotation_z((180_f32).to_radians())),
                EnemyBullet {
                    // oglisca_izhodisce: oglisca,
                    // radij: ENEMYBULLET_RADIUS
                },
                Enemy {
                    oglisca_izhodisce: oglisca,
                    radij: ENEMYBULLET_RADIUS
                },
            ));
        }
    }
}



pub fn enemybullet_movement(
    mut enemybullet_query: Query<&mut Transform, With<EnemyBullet>>, 
    time: Res<Time>
) {
    let bullet_direction = Vec3::new(0.0, -1.0, 0.0);
    for mut enemybullet_transform in enemybullet_query.iter_mut() {
        enemybullet_transform.translation += bullet_direction * ENEMYBULLET_SPEED * time.delta_secs();
    }
}

pub fn enemybullets_despawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemybullet_query: Query<(Entity, &Transform), With<EnemyBullet>>
) {
    let window = window_query.get_single().unwrap();

    let min_y = - window.height() / 2.0 - 20.0;

    for (enemybullet_entity, enemybullet_transform) in enemybullet_query.iter_mut() {
        if enemybullet_transform.translation.y < min_y {
            commands.entity(enemybullet_entity).despawn();
        }
    }
}

pub fn despawn_all_enemybullets(
    mut commands: Commands,
    enemybullet_query: Query<Entity, With<EnemyBullet>>,
) {
    for enemybullet in enemybullet_query.iter() {
        commands.entity(enemybullet).despawn();
    }
}