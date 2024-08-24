use bevy::prelude::*;

use crate::components::{Position, Size};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

pub fn pos_translation(mut window: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
    // x = 5
    // x / 10 * 200 - 200 / 2 = 300
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game; /* 64 if the window width is 1280 */

        /* the window size is divided in half because coord starts at bottom-left */
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }

    let primary_window = window.single_mut();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, primary_window.width(), ARENA_HEIGHT as f32),
            convert(pos.y as f32, primary_window.height(), ARENA_WIDTH as f32),
            0.0,
        )
    }
}

pub fn size_scaling(mut window: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>) {
    let primary_window = window.single_mut();

    // If something has a width of 1 in a grid of 40 and the window width is 400px, it should have
    // a width of 10
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * primary_window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * primary_window.height(),
            1.0,
        )
    }
}
