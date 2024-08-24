use bevy::prelude::*;

use crate::components::{Food, Position, SnakeHead, Direction};
use crate::constants::{ARENA_WIDTH, ARENA_HEIGHT};
use crate::events::{GameOverEvent, GrowthEvent};
use crate::resources::{LastTailPosition, SnakeSegments};
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
    last_tail_pos: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments.push(spawn_snake_segment(&mut commands, last_tail_pos.0.unwrap()));
    }
}

pub fn snake_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let key_pressed = keyboard_input.get_just_pressed().next();

        let current_dir: Direction = match key_pressed {
            Some(KeyCode::ArrowDown) => Direction::Down,
            Some(KeyCode::ArrowUp) => Direction::Up,
            Some(KeyCode::ArrowLeft) => Direction::Left,
            Some(KeyCode::ArrowRight) => Direction::Right,
            _ => head.direction,
        };

        let opposite_dir = head.direction.opposite();

        if current_dir != opposite_dir {
            head.direction = current_dir;
        }
    }
}

pub fn snake_movement(
    time: Res<Time>,
    segments: ResMut<SnakeSegments>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut last_tail_pos: ResMut<LastTailPosition>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, mut head)) = heads.iter_mut().next() {
        head.movement_timer.tick(time.delta());

        if !head.movement_timer.finished() {
            return;
        }

        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        }

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 == ARENA_WIDTH
            || head_pos.y as u32 == ARENA_HEIGHT
            || segment_positions.contains(&head_pos)
        {
            game_over_writer.send(GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });

        *last_tail_pos = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}
