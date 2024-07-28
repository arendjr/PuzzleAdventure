use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Deref for AnimationTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AnimationTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
pub struct MovementTimer(Timer);

impl Default for MovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

impl Deref for MovementTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MovementTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
