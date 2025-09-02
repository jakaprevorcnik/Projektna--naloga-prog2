use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::enemies::systems::{create_vertex_vector};
use crate::enemies::components::Enemy;

use crate::enemies::enemyships::{components::*, resources::{ENEMYSHIP_MAX_SPAWN_TIME, ENEMYSHIP_MIN_SPAWN_TIME}};
use crate::ui::score::resources::GameTime;

use crate::enemies::enemyships::resources::EnemyShipSpawnTimers;

const ENEMYSHIP_VERTICAL_SPEED: f32 = 50.0;
const ENEMYSHIP_HORIZONTAL_SPEED: f32 = 120.0;

const ENEMYSHIP_RADIUS: f32 = 67.0;
const OGLISCA_ENEMYSHIP_SLIKA: [Vec2; 16] = [Vec2::new(17., 4.), Vec2::new(21.5, 0.), Vec2::new(104.5, 0.),
    Vec2::new(109., 4.), Vec2::new(118., 28.), Vec2::new(126., 60.), Vec2::new(126., 73.),
    Vec2::new(123., 82.), Vec2::new(105., 106.), Vec2::new(96., 108.), Vec2::new(29., 108.),
    Vec2::new(21., 106.), Vec2::new(3., 82.), Vec2::new(0., 73.), Vec2::new(0., 60.), Vec2::new(8., 28.)];
const CENTER_ENEMYSHIP_SLIKA: Vec2 = Vec2::new(63., 54.);

const ENEMYBULLET_SHOOT_TIME: f32 = 1.8;



pub fn tick_enemyship_spawn_timer(
    mut enemyship_spawn_timers: ResMut<EnemyShipSpawnTimers>,
    time: Res<Time>
) {
    enemyship_spawn_timers.timer.tick(time.delta());
}

pub fn reset_enemyship_spawn_timers(
    mut enemyship_spawn_timers: ResMut<EnemyShipSpawnTimers>
) {
    enemyship_spawn_timers.to_default(); 
}

pub fn spawn_enemyships_over_time(
    mut enemyship_spawn_timers: ResMut<EnemyShipSpawnTimers>,
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game_time: Res<GameTime>
) {
    if enemyship_spawn_timers.timer.finished() {
        spawn_enemyship(commands, window_query, asset_server);
        
        let mut new_time = ENEMYSHIP_MAX_SPAWN_TIME - random::<f32>() * 0.1 * game_time.get_time();
        if new_time < ENEMYSHIP_MIN_SPAWN_TIME {
            new_time = ENEMYSHIP_MIN_SPAWN_TIME;
        }

        enemyship_spawn_timers.set_new_timer(new_time);
        enemyship_spawn_timers.timer.unpause();

        println!("{new_time}");
    }
}

fn spawn_enemyship(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * 512.0 - 256.; 
        let ship_y = window.height() / 2.0 + 60. / 2.0; // 60 je malo več kot polovica višine originalne slike ladje
        let coin_flip_direction = random::<bool>();
        let mut x_smer = 1.0;
        if coin_flip_direction {
            x_smer = -1.0;
        }
        let oglisca = create_vertex_vector(
            OGLISCA_ENEMYSHIP_SLIKA.to_vec(), CENTER_ENEMYSHIP_SLIKA, (180_f32).to_radians(), 0.5);
    
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
                    // radij : ENEMYSHIP_RADIUS
                },
                Enemy {
                    oglisca_izhodisce : oglisca,
                    radij : ENEMYSHIP_RADIUS
                },
            )
        );            
}




pub fn enemyship_movement(
    mut enemyship_query: Query<(&mut Transform, &EnemyShip)>, 
    time: Res<Time>,
) {
    let enemyship_vertical = Vec3::new(0., -1., 0.);
    let enemyship_horizontal = Vec3::new(1.0, 0.0, 0.0);
    
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



