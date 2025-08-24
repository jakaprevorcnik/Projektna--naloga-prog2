use bevy::prelude::*;

use crate::AppState;
use crate::events::*;



pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
  }



// Kje in kako naj bo tale funkcija, še ne vem.
pub fn toggle_states(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<AppState>>,
) {
    match *app_state.get() {
        AppState::MainMenu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                simulation_state_next_state.set(AppState::Game);
                println!("New game started.");
            }
        }
        AppState::GameOver => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                simulation_state_next_state.set(AppState::Game);
                println!("New game started.");
            }

            if keyboard_input.just_pressed(KeyCode::Escape) {
                simulation_state_next_state.set(AppState::MainMenu);
                println!("Returned to main menu.");
            }
        }
        AppState::Game => {} // Ali naj bi obstajala možnost za return na main menu?
    }
}

// Kle nej bi ble sam menjave in ta komunikacija.

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
        app_state_next_state.set(AppState::GameOver);
        println!("Entered AppState::GameOver");
    }
}


#[derive(Component)]
pub struct GameText;

pub fn display_score_game_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
    // še ne obstaja pravi score
    let score: u32 = 0;
    let high_score: u32 = 0;
    

    commands.spawn((
        Text2d::new(format!("Your score:\n{}", score)),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Right),
        Transform::from_xyz(-384., 48., -1.), //Ne vem, kako narediti transform glede na desen rob.
        GameText,
    ));  

        commands.spawn((
        Text2d::new(format!("High score:\n{}", high_score)),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Right),
        Transform::from_xyz(-384., -48., -1.),
        GameText,
    ));  
}

pub fn despawn_score_game_text(
  mut commands: Commands,
  text_query: Query<Entity, With<GameText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}





// SAT convex polygon collision detection
pub fn sat_collision_detection(veckotnik1: &Vec<Vec2>, veckotnik2: &Vec<Vec2>) -> bool { // TIPI ?!?!?!
    
    // zdele ne vem, kako z referencami delat
    let mut veckotnik1 = & *veckotnik1;
    let mut veckotnik2 = & *veckotnik2;

    for p in 0..2 {
        if p == 1 {
            let t = & *veckotnik1;
            veckotnik1 = veckotnik2;
            veckotnik2 = t;
        }
    
    let velikost1 = veckotnik1.len();

    for (i, vec) in veckotnik1.iter().enumerate() {
        let j = (i + 1) % velikost1;
        let os_projekcije = (veckotnik1[j] - vec).perp(); // a to res nardi normalo na to stranico?

        let min_r1 = &mut (f32::INFINITY);
        let max_r1 = &mut (-(f32::INFINITY));  // Pozabila sem, kako uporabljati reference .... ?!?!
        for vec1 in veckotnik1.iter() {
            let q = os_projekcije.dot(*vec1);

            *min_r1 = q.min(*min_r1);
            *max_r1 = q.max(*max_r1);
        }

        let min_r2 = &mut (f32::INFINITY);
        let max_r2 = &mut (-(f32::INFINITY));
        for vec2 in veckotnik2.iter() {
            let q = os_projekcije.dot(*vec2);

            *min_r2 = q.min(*min_r2);
            *max_r2 = q.max(*max_r2);
        }

        if !((*max_r1 - *min_r2).is_sign_positive() && (*max_r2 - *min_r1).is_sign_positive()) {
            return false
        }
    }
    
    }

    return true
}