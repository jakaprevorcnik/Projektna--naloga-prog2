use bevy::prelude::*;

// dodajam zaradi collision sem ? Morda v systems? Struktura je tukaj malo za premislit, zdaj hitim.
pub const PLAYER_HEIGHT: f32 = 82.;
pub const PLAYER_WIDTH: f32 = 100.;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bullet{}