use bevy::prelude::*;

use crate::components::{Food, SnakeSegment};
use crate::resources::SnakeSegments;
use crate::events::GameOverEvent;
use crate::systems::spawning::spawn_snake;

pub fn game_over(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if game_over_reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}
