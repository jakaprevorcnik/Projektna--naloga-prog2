use bevy::prelude::*;

use crate::AppState;
use crate::events::*;



pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
  }



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


// Kle nej bi ble sam menjave in ta komunikacija. Ostalo je v gameover, trenutno.






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