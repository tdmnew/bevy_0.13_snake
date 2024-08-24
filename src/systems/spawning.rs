use bevy::prelude::*;
use rand::random;

use crate::constants::{
    ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR, SNAKE_HEAD_COLOR, SNAKE_SEGMENT_COLOR,
};
use crate::components::{Food, Position, Size, SnakeHead, SnakeSegment};
use crate::resources::{FoodSpawnTimer, SnakeSegments};

pub fn spawn_snake_head(commands: &mut Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead::default())
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(1.0))
        .id() /* returns the entity so callers can use it */
}

pub fn spawn_snake_segment(commands: &mut Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id() /* returns the entity so callers can use it */
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        spawn_snake_head(&mut commands, Position { x: 3, y: 3 }),
        spawn_snake_segment(&mut commands, Position { x: 3, y: 3 }),
    ]);
}

pub fn spawn_food(mut commands: Commands, time: Res<Time>, mut config: ResMut<FoodSpawnTimer>) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .insert(Size::square(1.0));
    }
}
