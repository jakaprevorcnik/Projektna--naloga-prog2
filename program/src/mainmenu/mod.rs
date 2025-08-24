use bevy::prelude::*;

mod systems;
mod components;

use systems::*;
use crate::AppState;

use crate::gameover::resources::{VisibleTextTimer, HiddenTextTimer};
use crate::gameover::systems::{tick_vh_text_timers, gameover_text_toggle_visibility};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisibleTextTimer>();
        app.init_resource::<HiddenTextTimer>();
        app.add_systems(OnEnter(AppState::MainMenu), 
        (display_mainmenu_text, display_score_mainmenu_text, spawn_main_menu_screen));
        app.add_systems(Update, (tick_vh_text_timers, gameover_text_toggle_visibility)
            .run_if(in_state(AppState::MainMenu))); //to je vse zdaj skupaj nametano. Verjetno bi morala narediti skupen UI modul in plugin.
        app.add_systems(OnExit(AppState::MainMenu), (despawn_mainmenu_text, despawn_main_menu_screen_image));
    }
}

// Je za združit v UI/state modul z gameover in pa s systemi v crate::systems za display score-a in highscore-a.