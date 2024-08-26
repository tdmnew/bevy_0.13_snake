use bevy::prelude::*;

use crate::components::{Food, Position, SnakeHead, SnakeSegment};
use crate::events::{GameOverEvent, GrowthEvent};
use crate::resources::{LastTailPosition, SnakeSegments};
use crate::systems::spawning::{spawn_snake, spawn_snake_segment};

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

pub fn snake_eat(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if head_pos == food_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_grow(
    mut commands: Commands,
    mut growth_reader: EventReader<GrowthEvent>,
    mut segments: ResMut<SnakeSegments>,
    last_tail_pos: Res<LastTailPosition>,
) {
    if growth_reader.read().next().is_some() {
        segments.push(spawn_snake_segment(&mut commands, last_tail_pos.0.unwrap()));
    }
}
