use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::meteors::components::*;
use crate::player::components::*;

use super::resources::MeteorSpawnTimer;

pub const METEOR_SPEED: f32 = 100.0;

// To je radij približno včrtanega kroga.
// Ko bo collision detection s pravimi točkami in
// bo program najprej preveril za kroge, bova samo povečala
// ta radij, da bo vsaj toliko, da zaobjame cel meteor,
// in dodala dodatno preverjanje za večkotnike za tiste, ki so dovolj blizu.
pub const METEOR_RADIUS: f32 = 102.0;



pub fn tick_meteor_spawn_timer(
    mut meteor_spawn_timer: ResMut<MeteorSpawnTimer>,
    time: Res<Time>) {
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
            let meteor_y = window.height() / 2.0 + 215. * random_scale / 2.0; // 211 nej bi bila višina originalne slike meteorja. Da se pojavi lepše.
            // Te naračunane float-e bi lahko (morala) še dati v konstante ...
        
            commands.spawn(
                (
                    Sprite{
                        image: asset_server.load("sprites/spaceMeteors_001.png"),
                        ..Default::default()
                    },
                    Transform::from_xyz(random_x, meteor_y, 0.0).with_scale(Vec3::splat(random_scale)),
                    Meteor {
                        radij : METEOR_RADIUS * random_scale
                    },
                )
            );            
        }
    }


pub fn meteor_movement(
    mut meteor_query: Query<(&mut Transform, &Meteor)>, 
    time: Res<Time>
) {
    for (mut meteor_transform, meteor) in meteor_query.iter_mut() {
        let meteor_direction = Vec3::new(0.0, -1.0, 0.0);
        meteor_transform.translation += meteor_direction * METEOR_SPEED * time.delta_secs();
    }
}

pub fn meteor_despawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meteor_query: Query<(Entity, &Transform), With<Meteor>>
) {
    let window = window_query.get_single().unwrap();

    // Kdaj se despawn-a.  (Saj ne vem, ali jih drugače ne despawn-a sam in ali to sploh potrebujemo,
    // ampak, če stvari izven okvirja ne despawn-a sam, ne vem, zakaj jih ne bi mi, da ne računa ves čas za vedno več enemyjev.
    // Za meteorje pa dajem, da jih enotno despavna, ko so na -211/2, da vse pokrije. Recimo na -110.)
    let min_y = - window.height() / 2.0 - 110.0;

    for (meteor_entity, meteor_transform) in meteor_query.iter_mut() {
        if meteor_transform.translation.y < min_y {
            commands.entity(meteor_entity).despawn();
        }
    }
}




// Ali naj bo collision system tukaj ali pri player-ju?
// Ali kaj pridobiva na učinkovitosti, če hkrati preverjava collision z metki (in posodablajava health)
// Zaenkrat je aproksimacija: meteorji-krogi in player-pravokotnik.
// Glej https://www.jeffreythompson.org/collision-detection/circle-rect.php, ta algoritem je.
pub fn check_collsion_meteor_player_rough(
    // mut commands: Commands,
    mut meteor_query: Query<(Entity, &Transform, &Meteor), With<Meteor>>,
    player_query: Query<&Transform, With<Player>>
) {
    if let Ok(player_transform) = player_query.get_single() {
        let levi_rob = player_transform.translation.x - PLAYER_WIDTH / 4.0; // je še scale-ano za pol, to bi tud morala dat v const vse.
        let desni_rob = player_transform.translation.x + PLAYER_WIDTH / 4.0;
        let zgornji_rob = player_transform.translation.y + PLAYER_HEIGHT / 4.0;
        let spodnji_rob = player_transform.translation.y - PLAYER_WIDTH / 4.0;
        
        for (meteor_entity, meteor_tranform, meteor) in meteor_query.iter_mut() {
            let met_x0 = meteor_tranform.translation.x;
            let met_y0 = meteor_tranform.translation.y; // koordinati središča

            let mut testX = met_x0;
            let mut testY = met_y0;

            if met_x0 < levi_rob {
                testX = levi_rob
            } else if met_x0 > desni_rob {
                testX = desni_rob
            }
            if met_y0 < spodnji_rob {
                testY = spodnji_rob
            } else if met_y0 > zgornji_rob {
                testY = zgornji_rob
            } 

            let distX = met_x0 - testX;
            let distY = met_y0 - testY;
            let distance_square = (distX*distX) + (distY*distY);

            if distance_square <= meteor.radij * meteor.radij {
                println!("Zadet si bil.")
            }
            // Zamujam, tako da ni zrihtano nič. Tako da je še za zlepšat tudi to, kar je napisano. V splošnem pa itak še milijon za premislt.
            // Vem, da je print slabo, ampak zdele nimam časa ugotavlat, kaj je dejansko treba narest.          
    
        }
    }

}