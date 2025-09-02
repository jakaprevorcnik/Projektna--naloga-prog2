use bevy::prelude::*;

const BULLET_SHOOT_TIMER: f32 = 0.2;

#[derive(Resource)]
pub struct BulletShootTimer {
    pub timer : Timer
}

impl Default for BulletShootTimer {
    fn default() -> Self {
        BulletShootTimer {
            timer : Timer::from_seconds(BULLET_SHOOT_TIMER, TimerMode::Once)
        }
    }
}