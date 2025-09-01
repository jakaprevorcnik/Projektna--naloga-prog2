use bevy::prelude::*;

use rand::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::components::Player;
use crate::ui::score::resources::{GameTime, Score};

const ASTRONAUT_SIZE: f32 = 64.0;
const COLLECTION_DISTANCE: f32 = 32.0; //polovicna razdalja astronavta (da se priblizno zaletimo vanj)
const ASTRONAUT_SPEED: f32 = 300.0; 
 
pub fn tick_astronaut_spawn_timer(
    mut astronaut_spawn_timer: ResMut<AstronautSpawnTimer>,
    time: Res<Time>,
) {
    astronaut_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_astronauts_over_time(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
    astronaut_spawn_timer: Res<AstronautSpawnTimer>,
) {
    if astronaut_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        
        let random_scale = random::<f32>() * 0.5 + 0.5;
        let random_x = random::<f32>() * 512.0 - 256.;
        let random_y = window.height() / 2.0 + 215. * random_scale / 2.0;
        
        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/spaceAstronauts_001.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Astronaut::default(),
        ));
    }
}

pub fn check_astronaut_collection(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Astronaut>)>,
    astronaut_query: Query<(Entity, &Transform, &Astronaut), (With<Astronaut>, Without<Player>)>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (astronaut_entity, astronaut_transform, astronaut) in astronaut_query.iter() {
            let distance = player_transform.translation.distance(astronaut_transform.translation);
            
            if distance < COLLECTION_DISTANCE {
                score.score += astronaut.points_value;

                commands.entity(astronaut_entity).despawn();
            }
        }
    }
}

pub fn astronaut_movement(
    mut astronaut_query: Query<(&mut Transform, &Astronaut)>, 
    time: Res<Time>,
    game_time: Res<GameTime>
) {
    let astronaut_direction = Vec3::new(0.0, -1.0, 0.0);
    
    
    // let time_intervals = game_time.get_time() / 30.0;  //ce zelimo da se spreminja hitrost s casom
    // let speed_increase = time_intervals * 50.0;
     let current_speed =  ASTRONAUT_SPEED; //+ speed_increase;
    
    for (mut astronaut_transform, _meteor) in astronaut_query.iter_mut() {
        astronaut_transform.translation += astronaut_direction * current_speed * time.delta_secs();
    }
}

pub fn astronaut_despawn(
    mut commands: Commands,
    astronaut_query: Query<(Entity, &Transform), With<Astronaut>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();
    
    for (astronaut_entity, astronaut_transform) in astronaut_query.iter() {
        let translation = astronaut_transform.translation;
        
        if translation.y < -window.height()
            || translation.y > window.height() * 2.0
            || translation.x < -window.width() 
            || translation.x > window.width() * 2.0
        {
            commands.entity(astronaut_entity).despawn();
        }
    }
}

pub fn despawn_all_astronauts(
    mut commands: Commands,
    astronaut_query: Query<Entity, With<Astronaut>>,
) {
    for astronaut_entity in astronaut_query.iter() {
        commands.entity(astronaut_entity).despawn();
    }
}
