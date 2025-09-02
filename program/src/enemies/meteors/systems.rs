use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::enemies::meteors::components::*;
use crate::enemies::components::Enemy;
use crate::enemies::systems::create_vertex_vector;
use crate::ui::score::resources::GameTime;
use super::resources::MeteorSpawnTimer;

pub const METEOR_SPEED: f32 = 300.0;

pub const METEOR_RADIUS: f32 = 107.5;

// ARRAY ALI VECTOR ???
const OGLISCA_METEORJA_SLIKA: [Vec2; 10] = [Vec2::new(35., -204.), Vec2::new(105., -211.),
     Vec2::new(173., -191.), Vec2::new(211., -145.), Vec2::new(215., -74.), Vec2::new(171., -19.), 
     Vec2::new(111.5, 0.), Vec2::new(37., -23.5), Vec2::new(0., -72.), Vec2::new(3.5, -136.)];
const CENTER_METEORJA_SLIKA: Vec2 = Vec2::new(107.5, -105.5);


pub fn despawn_all_meteors(
    mut commands: Commands,
    meteor_query: Query<Entity, With<Meteor>>     
) {
    for meteor_entity in meteor_query.iter() {
        commands.entity(meteor_entity).despawn();
    }
}


pub fn tick_meteor_spawn_timer(
    mut meteor_spawn_timer: ResMut<MeteorSpawnTimer>,
    time: Res<Time>
) {
    meteor_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_meteors_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    meteor_spawn_timer: Res<MeteorSpawnTimer>
) {
        if meteor_spawn_timer.timer.finished() {
            let window = window_query.get_single().unwrap();

            let random_scale = random::<f32>() * 0.5 + 0.5;
            let random_x = random::<f32>() * 512.0 - 256.; 
            let meteor_y = window.height() / 2.0 + 215. * random_scale / 2.0; // + malo več kot polovica višine meteorja. Podobono pri ostalih sprite-ih.
            let kot = (random::<f32>() * 360.).to_radians();

            let oglisca = create_vertex_vector(
                OGLISCA_METEORJA_SLIKA.to_vec(), CENTER_METEORJA_SLIKA, kot, random_scale);
        
            commands.spawn(
                (
                    Sprite{
                        image: asset_server.load("sprites/spaceMeteors_001.png"),
                        ..Default::default()
                    },
                    Transform::from_xyz(random_x, meteor_y, 0.0).with_scale(Vec3::splat(random_scale))
                        .with_rotation(Quat::from_rotation_z(kot)),
                    Meteor,
                    Enemy {
                        radij : METEOR_RADIUS * random_scale,
                        oglisca_izhodisce : oglisca,
                    },
                )
            );            
        }
    }


pub fn meteor_movement(
    mut meteor_query: Query<(&mut Transform, &Meteor)>, 
    time: Res<Time>,
    game_time: Res<GameTime>
) {
    let meteor_direction = Vec3::new(0.0, -1.0, 0.0);
    
    let time_intervals = game_time.get_time() / 30.0; 
    let speed_increase = time_intervals * 50.0;
    let current_speed = METEOR_SPEED + speed_increase;
    
    for (mut meteor_transform, _meteor) in meteor_query.iter_mut() {
        meteor_transform.translation += meteor_direction * current_speed * time.delta_secs();
    }
}


pub fn meteor_despawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    meteor_query: Query<(Entity, &Transform), With<Meteor>>
) {
    let window = window_query.get_single().unwrap();

    let min_y = - window.height() / 2.0 - 110.0;

    for (meteor_entity, meteor_transform) in meteor_query.iter() {
        if meteor_transform.translation.y < min_y {
            commands.entity(meteor_entity).despawn();
        }
    }
}

