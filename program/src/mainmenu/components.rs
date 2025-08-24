use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuText;

// Ali naj združiva komponente, resource-e in nekatere system-e za gameover in mainmenu?
// Morda modul UI? in znotraj tega main menu in game over. ?  
// In gre v ta modul tudi izpis score-a v Game state-u itd.

#[derive(Component)]
pub struct MenuImage;