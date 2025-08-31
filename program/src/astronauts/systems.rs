use bevy::prelude::*;
use rand::prelude::*;
use super::components::*;
use super::resources::*;
use crate::player::components::Player;
use crate::resources::Score;

const ASTRONAUT_SIZE: f32 = 64.0;
const COLLECTION_DISTANCE: f32 = 32.0; //polovicna razdalja astronavta (da se priblizno zaletimo vanj)

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
        
        
        let random_x = random::<f32>() * (window.width() - ASTRONAUT_SIZE) + ASTRONAUT_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - ASTRONAUT_SIZE) + ASTRONAUT_SIZE / 2.0;
        
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
