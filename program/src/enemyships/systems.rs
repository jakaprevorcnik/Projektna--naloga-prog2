use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::enemyships::components::*;
use crate::ui::score::resources::GameTime;

use crate::enemyships::resources::EnemyShipSpawnTimer;

const ENEMYSHIP_VERTICAL_SPEED: f32 = 50.0;
const ENEMYSHIP_HORIZONTAL_SPEED: f32 = 120.0;
// pub const ENEMYSHIP_RADIUS: f32 = ;

// const OGLISCA
// const CENTER

const ENEMYBULLET_SHOOT_TIME: f32 = 1.8;
const ENEMYBULLET_SPEED: f32 = 200.0;



pub fn tick_enemyship_spawn_timer(
    mut enemyship_spawn_timer: ResMut<EnemyShipSpawnTimer>,
    time: Res<Time>
) {
    enemyship_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemyships_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemyship_spawn_timer: Res<EnemyShipSpawnTimer>
) {
        if enemyship_spawn_timer.timer.finished() {
            let window = window_query.get_single().unwrap();

            let random_x = random::<f32>() * 512.0 - 256.; 
            let ship_y = window.height() / 2.0 + 60. / 2.0; // 60 je malo več kot polovica višine originalne slike ladje
            let coin_flip_direction = random::<bool>();
            let mut x_smer = 1.0;
            if coin_flip_direction {
                x_smer = -1.0;
            }

            // let oglisca = ;
        
            commands.spawn(
                (
                    Sprite{
                        image: asset_server.load("sprites/spaceShips_004.png"),
                        ..Default::default()
                    },
                    Transform::from_xyz(random_x, ship_y, 0.0).with_scale(Vec3::splat(0.5))
                        .with_rotation(Quat::from_rotation_z((180_f32).to_radians())),
                    EnemyShip {
                        x_smer : x_smer,
                        bullet_shoot_timer : Timer::from_seconds(ENEMYBULLET_SHOOT_TIME, TimerMode::Repeating),
                        // oglisca_izhodisce : oglisca,
                    },
                )
            );            
        }
    }




pub fn enemyship_movement(
    mut enemyship_query: Query<(&mut Transform, &EnemyShip)>, 
    time: Res<Time>,
    // game_time: Res<GameTime>
) {
    let enemyship_vertical = Vec3::new(0., -1., 0.);
    let enemyship_horizontal = Vec3::new(1.0, 0.0, 0.0);
    
    
    // let time_intervals = game_time.get_time() / 30.0; 
    // let speed_increase = time_intervals * 50.0;
    // let current_speed = ENEMYSHIP_VERTICAL_SPEED + speed_increase;
    
    for (mut enemyship_transform, enemyship) in enemyship_query.iter_mut() {
        enemyship_transform.translation += enemyship_vertical * ENEMYSHIP_VERTICAL_SPEED * time.delta_secs(); // vertical
        enemyship_transform.translation += enemyship_horizontal * enemyship.x_smer * ENEMYSHIP_HORIZONTAL_SPEED * time.delta_secs(); // horizontal
    }
}

pub fn confine_enemyship_movement(
    mut enemyship_query: Query<(&mut Transform, &mut EnemyShip)>,
) {
    for (mut enemyship_transform, mut enemyship) in enemyship_query.iter_mut() {
        let x_min = -256.;
        let x_max = 256.;

        let mut translation = enemyship_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
            enemyship.x_smer = enemyship.x_smer * -1.0;
        } else if translation.x > x_max {
            translation.x = x_max;
            enemyship.x_smer = enemyship.x_smer * -1.0;
        }

        enemyship_transform.translation = translation;
    }
}



pub fn enemyship_despawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemyship_query: Query<(Entity, &Transform), With<EnemyShip>>
) {
    let window = window_query.get_single().unwrap();

    let min_y = - window.height() / 2.0 - 70.0;

    for (enemyship_entity, enemyship_transform) in enemyship_query.iter_mut() {
        if enemyship_transform.translation.y < min_y {
            commands.entity(enemyship_entity).despawn();
        }
    }
}


pub fn despawn_all_enemyships(
    mut commands: Commands,
    mut ship_query: Query<Entity, With<EnemyShip>>     
) {
    for ship_entity in ship_query.iter() {
        commands.entity(ship_entity).despawn();
    }
}



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
                EnemyBullet {},
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