use crate::components::Position;
use bevy::prelude::*;

/**
 *
 * Resources
 * 
 **/
#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource)]
pub struct FoodSpawnTimer {
    pub timer: Timer,
}
impl Default for FoodSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
