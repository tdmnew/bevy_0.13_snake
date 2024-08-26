use bevy::prelude::*;

use crate::components::{Food, Position, SnakeHead};
use crate::resources::{LastTailPosition, SnakeSegments};
use crate::events::GrowthEvent;
use crate::systems::spawning::spawn_snake_segment;

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

