use bevy::prelude::*;

use crate::components::{Direction, Position, SnakeHead};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::events::GameOverEvent;
use crate::resources::{LastTailPosition, SnakeSegments};

pub fn snake_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let key_pressed = keyboard_input.get_just_pressed().next();

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

        let snake_segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        snake_segment_positions
            .iter()
            /* create (Entity<SnakeSegment>, Position) */
            .zip(segments.iter().skip(1)) 
            /* get Entity for SnakeSegment from Vec<Position> and assign its associated Position to it */
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos
            });

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
            || snake_segment_positions.contains(&head_pos)
        {
            game_over_writer.send(GameOverEvent);
        }

        *last_tail_pos = LastTailPosition(Some(*snake_segment_positions.last().unwrap()));
    }
}
