use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::GameOver;
use crate::player::components::*;
use crate::meteors::components::*;

pub const PLAYER_SPEED: f32 = 500.0;

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
}

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
// Zaenkrat je aproksimacija: meteorji-krogi in player-pravokotnik.
// Glej https://www.jeffreythompson.org/collision-detection/circle-rect.php, ta algoritem je.
pub fn check_collsion_meteor_player_rough(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut meteor_query: Query<(&Transform, &Meteor), With<Meteor>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        let levi_rob = player_transform.translation.x - PLAYER_WIDTH / 4.0; // je še scale-ano za pol, to bi tud morala dat v const vse.
        let desni_rob = player_transform.translation.x + PLAYER_WIDTH / 4.0;
        let zgornji_rob = player_transform.translation.y + PLAYER_HEIGHT / 4.0;
        let spodnji_rob = player_transform.translation.y - PLAYER_WIDTH / 4.0;
        
        for (meteor_tranform, meteor) in meteor_query.iter_mut() {
            let met_x0 = meteor_tranform.translation.x;
            let met_y0 = meteor_tranform.translation.y; // koordinati središča

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

            if distance_square <= meteor.radij * meteor.radij {
                println!("Zadet si bil.");
                commands.entity(player_entity).despawn();
                // score-a še nimava.
                game_over_event_writer.send(GameOver { score : 0 });
            }
            // Zamujam, tako da ni zrihtano nič. Tako da je še za zlepšat tudi to, kar je napisano. V splošnem pa itak še milijon za premislt.
            // Vem, da je print slabo, ampak zdele nimam časa ugotavlat, kaj je dejansko treba narest.          
    
        }
    }

}