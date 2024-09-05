use bevy::prelude::*;

use crate::components::{Direction, Position, SnakeHead};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::events::GameOverEvent;
use crate::resources::{LastTailPosition, SnakeSegments};

/// Get Vec<Position> from SnakeSegment(<Vec<Entity>>)
/// ex. (when snake is moving downwards) [
///  SnakeHead::Position { x: 3: y: 4 },
///  SnakeSegment::Position { x: 3: y: 5 },
///  SnakeSegment::Position { x: 3: y: 6 }
/// ]
fn get_segment_positions(
    segments: &Vec<Entity>,
    positions_q: &mut Query<&mut Position>,
) -> Vec<Position> {
    segments
        .iter()
        .map(|e| *positions_q.get_mut(*e).unwrap())
        .collect::<Vec<Position>>()
}

/// Update the position of each SnakeSegment to the SnakeHead/SnakeSegment position in front of it
/// ex. Taking the original Vec<Entity> and performing .zip() with .skip(1): [
///   (Position { x: 3, y: 4 }, Entity { x: 3: y: 5 }),
///   (Position { x: 3, y: 5 }, Entity { x: 3: y: 6 })
/// ]
/// Segments following are now:
/// [SnakeSegment::Position { x: 3, y: 4 },  and SnakeSegment::Position { x: 3, y: 5 }]
fn update_segment_positions(
    original_positions: &Vec<Position>,
    segments: &Vec<Entity>,
    positions_q: &mut Query<&mut Position>,
) {
    original_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(pos, segment_entity)| *positions_q.get_mut(*segment_entity).unwrap() = *pos);
}

// Translate keyboard_input into Direction for the SnakeHead
pub fn snake_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let key_pressed = keyboard_input.get_just_pressed().next();

        // keep track of snake direction and last user input independently
        let next_dir: Direction = match key_pressed {
            Some(KeyCode::ArrowDown) => Direction::Down,
            Some(KeyCode::ArrowUp) => Direction::Up,
            Some(KeyCode::ArrowLeft) => Direction::Left,
            Some(KeyCode::ArrowRight) => Direction::Right,
            _ => head.direction,
        };

        let opposite_dir = head.direction.opposite();

        if next_dir != opposite_dir {
            head.direction = next_dir;
        }
    }
}

/// First updates each SnakeSegment Position to follow where the SnakeHead went last tick
/// Then updates the SnakeHead Position with the new Position this tick
pub fn snake_movement(
    time: Res<Time>,
    segments: Res<SnakeSegments>,
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

        let segment_positions = get_segment_positions(&segments, &mut positions);

        update_segment_positions(&segment_positions, &segments, &mut positions);

        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Right => {
                head_pos.x += 1;
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

        *last_tail_pos = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}
