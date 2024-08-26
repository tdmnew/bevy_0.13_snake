use bevy::prelude::*;

use crate::systems::{events::*, interaction::*, movement::*, spawning::*, translation_scale::*};
use crate::events::{GameOverEvent, GrowthEvent};
use crate::resources::{FoodSpawnTimer, LastTailPosition, SnakeSegments};

mod components;
mod constants;
mod events;
mod resources;
mod systems;

/**
 *  
 * Startup
 *
 **/
fn setup_camera(mut commands: Commands) {
    // Create a new 2D camera
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake game".to_string(),
                resolution: Vec2::new(500.0, 500.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(Update, spawn_food)
        .add_systems(PostUpdate, (size_scaling, pos_translation))
        .add_systems(
            FixedUpdate,
            (
                (snake_input, snake_movement).chain(),
                (
                    snake_eat.after(snake_movement),
                    snake_grow.after(snake_movement),
                    game_over.after(snake_movement),
                )
                    .chain(),
            ),
        )
        .insert_resource(FoodSpawnTimer::default())
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .run();
}
