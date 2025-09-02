use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::GameOver;
use crate::player::components::*;
use crate::meteors::components::*;
use crate::systems::sat_collision_detection;
use crate::ui::score::resources::{Score, HighScore};

pub const PLAYER_SPEED: f32 = 500.0;
pub const BULLET_SPEED: f32 = 700.0;

const OGLISCA_LADJE_SLIKA: [Vec2; 10] = [Vec2::new(28., 0.), Vec2::new(72., 0.), Vec2::new(90., 18.), Vec2::new(100., 74.),
    Vec2::new(94., 81.), Vec2::new(90., 82.), Vec2::new(14., 82.), Vec2::new(6., 81.),
    Vec2::new(0., 74.), Vec2::new(10., 18.)];
const CENTER_LADJE_SLIKA: Vec2 = Vec2::new(50., 41.);

const OGLISCA_METKA_SLIKA: [Vec2; 12] = [Vec2::new(0., 5.), Vec2::new(1., 3.), Vec2::new(3., 1.), Vec2::new(9., 1.),
    Vec2::new(11., 3.), Vec2::new(12., 5.), Vec2::new(12., 18.), Vec2::new(11., 24.), Vec2::new(10.5, 25.),
    Vec2::new(1.5, 25.), Vec2::new(1., 24.), Vec2::new(0., 18.)];
const CENTER_METKA_SLIKA: Vec2 = Vec2::new(6., 12.5);
const BULLET_WIDTH: f32 = 12.0;
const BULLET_HEIGHT: f32 = 25.0;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/spaceShips_008.png"),
                ..Default::default()
            },
            Transform::from_xyz(0.0, - (window.height() / 2.5), 0.0).with_scale(Vec3::splat(0.5)), // ce das pr y 1.5 namest 2.5 je cis na dnu, sam se ga vid samo k mas cez cel zaslon sliko.
            Player {},
        )
    );
} // Za popravit je mogoče, ker če imaš fullscreen (kar sicer ni mišljeno), je do respawn-a vse čudn.

pub fn player_movement(keyboard_input: 
    Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let x_min = 0.0 - 256.;
        let x_max = 0.0 + 256.;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation;
    }
}



// Ali naj bo collision system tukaj ali pri player-ju? Pri PLAYER-JU
// Ali kaj pridobiva na učinkovitosti, če hkrati preverjava collision z metki (in posodablajava health)
// Da ne računamo vsakič sproti vsega?


pub fn check_collision_meteor_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut meteor_query: Query<(&Transform, &Meteor), With<Meteor>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        let levi_rob = player_transform.translation.x - PLAYER_WIDTH / 4.0; // je še scale-ano za pol, to bi tud morala dat v const vse.
        let desni_rob = player_transform.translation.x + PLAYER_WIDTH / 4.0;
        let zgornji_rob = player_transform.translation.y + PLAYER_HEIGHT / 4.0;
        let spodnji_rob = player_transform.translation.y - PLAYER_WIDTH / 4.0;
        
        for (meteor_transform, meteor) in meteor_query.iter_mut() {
            let met_x0 = meteor_transform.translation.x;
            let met_y0 = meteor_transform.translation.y;

            if check_collsion_meteor_player_aprox(met_x0, met_y0, meteor.radij, levi_rob, desni_rob, zgornji_rob, spodnji_rob) {
                // let mut oglisca_ladje = [Vec2::new(0., 0.); 10];
                // for (i, vec) in OGLISCA_LADJE_SLIKA.iter().enumerate() {
                //     oglisca_ladje[i] = vec - CENTER_LADJE_SLIKA + player_transform.translation.xy();
                // }

                // let mut oglisca_meteorja = [Vec2::new(0., 0.); 10];
                // for (i, vec) in meteor.oglisca_izhodisce.iter().enumerate() {
                //     oglisca_meteorja[i] = vec + Vec2::new(met_x0, met_y0);
                // }

                let mut oglisca_ladje = vec![];
                for vec in OGLISCA_LADJE_SLIKA.iter() {
                    oglisca_ladje.push(vec - CENTER_LADJE_SLIKA + player_transform.translation.xy())
                }

                let mut oglisca_meteorja = vec![];
                let pozicija_meteorja = Vec2::new(met_x0, met_y0);
                for vec in meteor.oglisca_izhodisce.iter() {
                    oglisca_meteorja.push(vec + pozicija_meteorja)
                }

                // if sat_collision_detection(&(oglisca_ladje.to_vec()), &(oglisca_meteorja.to_vec())) {
                if sat_collision_detection(&oglisca_ladje, &oglisca_meteorja) {
                    println!("Zadet si bil.");
                    commands.entity(player_entity).despawn();
                    
                    let final_score = score.get_score();
                    let is_new_high_score = high_score.update(final_score);
                    
                    if is_new_high_score {
                        println!("NEW HIGH SCORE! {}", final_score);
                    }
                    
                    game_over_event_writer.send(GameOver { score: final_score });                    
                }

            }
        }
}

}

// Aproksimacija je: meteorji-krogi in player-pravokotnik.
// Glej https://www.jeffreythompson.org/collision-detection/circle-rect.php, ta algoritem je.
fn check_collsion_meteor_player_aprox(
    met_x0: f32,
    met_y0: f32,
    met_radij: f32,
    levi_rob: f32,
    desni_rob: f32,
    zgornji_rob: f32,
    spodnji_rob: f32,
) -> bool {
    let mut test_x = met_x0;
    let mut test_y = met_y0;
    if met_x0 < levi_rob {
        test_x = levi_rob
    } else if met_x0 > desni_rob {
        test_x = desni_rob
    }
    if met_y0 < spodnji_rob {
        test_y = spodnji_rob
    } else if met_y0 > zgornji_rob {
        test_y = zgornji_rob
    } 
    let dist_x = met_x0 - test_x;
    let dist_y = met_y0 - test_y;
    let distance_square = (dist_x * dist_x) + (dist_y * dist_y);

    if distance_square <= met_radij * met_radij {
        return true;
    }

    return false
}




pub fn shoot_bullet(keyboard_input: 
    Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
){
    if keyboard_input.just_pressed(KeyCode::Space) {
        let ship_transform = player_query.single();
        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/spaceMissiles_015.png"),
                        ..Default::default()
            },
            Transform::from_xyz(
                ship_transform.translation.x,
                ship_transform.translation.y + 40.0,
                0.0,
                ),
            Bullet{}
        ));
    }
}

pub fn bullet_movement (
    mut bullet_query: Query<(&mut Transform, &Bullet)>, 
    time: Res<Time>
) {
    for (mut bullet_transform, _bullet) in bullet_query.iter_mut() {
        let bullet_direction = Vec3::new(0.0, 1.0, 0.0);
        bullet_transform.translation += bullet_direction * BULLET_SPEED * time.delta_secs();
    }
}

pub fn bullet_meteor_collision_system(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    meteor_query: Query<(Entity, &Transform, &Meteor), With<Meteor>>,
    mut score: ResMut<Score>,
) {
for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let levi_rob = bullet_transform.translation.x - BULLET_WIDTH / 4.0; // je še scale-ano za pol, to bi tud morala dat v const vse.
        let desni_rob = bullet_transform.translation.x + BULLET_WIDTH / 4.0;
        let zgornji_rob = bullet_transform.translation.y + BULLET_HEIGHT / 4.0;
        let spodnji_rob = bullet_transform.translation.y - BULLET_WIDTH / 4.0;
        
        for (meteor_entity, meteor_transform, meteor) in meteor_query.iter() {
            let met_x0 = meteor_transform.translation.x;
            let met_y0 = meteor_transform.translation.y;

            if check_collsion_meteor_player_aprox(met_x0, met_y0, meteor.radij, levi_rob, desni_rob, zgornji_rob, spodnji_rob) {
                // To je potrebno spremenit, da je pač square-circle_collision_aprox

                let mut oglisca_metka = vec![]; //Nekonsistentna uporaba angleščine in slovenščine. ...
                for vec in OGLISCA_METKA_SLIKA.iter() {
                    oglisca_metka.push(vec - CENTER_METKA_SLIKA + bullet_transform.translation.xy())
                }

                let mut oglisca_meteorja = vec![];
                let pozicija_meteorja = Vec2::new(met_x0, met_y0);
                for vec in meteor.oglisca_izhodisce.iter() {
                    oglisca_meteorja.push(vec + pozicija_meteorja)
                }

                // if sat_collision_detection(&(oglisca_ladje.to_vec()), &(oglisca_meteorja.to_vec())) {
                if sat_collision_detection(&oglisca_metka, &oglisca_meteorja) {
                    commands.entity(bullet_entity).despawn();
                    commands.entity(meteor_entity).despawn();
                    
                    // Add score for destroying meteor
                    
                    score.add_points(10);
                    
                    break;                
                }

            }
        }
}

}


pub fn bullets_despawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut bullet_query: Query<(Entity, &Transform), With<Bullet>>
) {
    let window = window_query.get_single().unwrap();

    let max_y = window.height() / 2.0 + 20.0;

    for (bullet_entity, bullet_transform) in bullet_query.iter_mut() {
        if bullet_transform.translation.y > max_y {
            commands.entity(bullet_entity).despawn();
        }
    }
}


pub fn despawn_all_bullets(
    mut commands: Commands,
    bullet_query: Query<Entity, With<Bullet>>     
) {
    for bullet_entity in bullet_query.iter() {
        commands.entity(bullet_entity).despawn();
    }
}
