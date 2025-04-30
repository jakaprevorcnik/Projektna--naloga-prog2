use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::meteors::components::*;

use super::resources::MeteorSpawnTimer;

pub const METEOR_SPEED: f32 = 100.0;


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
            let meteor_y = window.height() / 2.0 + 211. * random_scale / 2.0; // 215 nej bi bila višina originalne slike meteorja. Da se pojavi lepše.
        
            commands.spawn(
                (
                    Sprite{
                        image: asset_server.load("sprites/spaceMeteors_001.png"),
                        ..Default::default()
                    },
                    Transform::from_xyz(random_x, meteor_y, 0.0).with_scale(Vec3::splat(random_scale)),
                    Meteor {},
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
