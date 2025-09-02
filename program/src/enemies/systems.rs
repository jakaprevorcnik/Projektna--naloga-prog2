use bevy::prelude::*;

use crate::events::GameOver;
use crate::ui::score::resources::{Score, HighScore};
use crate::systems::sat_collision_detection;

use crate::player::{components::*, systems::{CENTER_LADJE_SLIKA, OGLISCA_LADJE_SLIKA}};
use crate::enemies::components::Enemy;

pub fn create_vertex_vector(
    mut vertex_vector: Vec<Vec2>,
    center: Vec2,
    kot_radiani: f32,
    skala: f32
) -> Vec<Vec2> {
    for vec in vertex_vector.iter_mut() {
        *vec = Vec2::from_angle(kot_radiani).rotate((*vec - center) * skala)
    };
    vertex_vector
}


pub fn check_collision_player_enemy(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut enemy_query: Query<(&Transform, &Enemy), With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        let levi_rob = player_transform.translation.x - PLAYER_WIDTH / 4.0;
        let desni_rob = player_transform.translation.x + PLAYER_WIDTH / 4.0;
        let zgornji_rob = player_transform.translation.y + PLAYER_HEIGHT / 4.0;
        let spodnji_rob = player_transform.translation.y - PLAYER_WIDTH / 4.0;
        
        for (enemy_transform, enemy) in enemy_query.iter_mut() {
            let enemy_x0 = enemy_transform.translation.x;
            let enemy_y0 = enemy_transform.translation.y;

            if circle_rectangle_collision_aprox(enemy_x0, enemy_y0, enemy.radij, levi_rob, desni_rob, zgornji_rob, spodnji_rob) {

                let mut oglisca_ladje = vec![];
                for vec in OGLISCA_LADJE_SLIKA.iter() {
                    oglisca_ladje.push(vec - CENTER_LADJE_SLIKA + player_transform.translation.xy())
                }

                let mut oglisca_enemyja = vec![];
                let pozicija_enemyja = Vec2::new(enemy_x0, enemy_y0);
                for vec in enemy.oglisca_izhodisce.iter() {
                    oglisca_enemyja.push(vec + pozicija_enemyja)
                }

                if sat_collision_detection(&oglisca_ladje, &oglisca_enemyja) {
                    commands.entity(player_entity).despawn();
                    
                    let final_score = score.get_score();
                    high_score.update(final_score);
                    
                    game_over_event_writer.send(GameOver { score: final_score });                    
                }

            }
        }
    }
}


pub fn circle_rectangle_collision_aprox(
    krog_x0: f32,
    krog_y0: f32,
    krog_radij: f32,
    levi_rob: f32,
    desni_rob: f32,
    zgornji_rob: f32,
    spodnji_rob: f32,
) -> bool {
    let mut test_x = krog_x0;
    let mut test_y = krog_y0;
    if krog_x0 < levi_rob {
        test_x = levi_rob
    } else if krog_x0 > desni_rob {
        test_x = desni_rob
    }
    if krog_y0 < spodnji_rob {
        test_y = spodnji_rob
    } else if krog_y0 > zgornji_rob {
        test_y = zgornji_rob
    } 
    let dist_x = krog_x0 - test_x;
    let dist_y = krog_y0 - test_y;
    let distance_square = (dist_x * dist_x) + (dist_y * dist_y);

    if distance_square <= krog_radij * krog_radij {
        return true;
    }

    return false
}
