use bevy::prelude::*;

use crate::components::{Position, Size};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

/// Shifts coordinates as centre of grid (0,0) is at the centre of window, rather than top-left
/// or bottom-left
pub fn pos_translation(mut window: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        /* the tile would be 64 pixels if the window width is 1280 */
        let tile_size = bound_window / bound_game;

        /* (pos / bound_game * bound_window) = scales grid pos to window size */
        /* (bound_window / 2.0) = subtract half window size to a centred origin in the window (e.g. 0,0) */
        /* (tile_size / 2.0) - move to centre of cell, not the bottom-left */
        pos * tile_size - (bound_window / 2.0) + (tile_size / 2.0)
    }

    let primary_window = window.single_mut();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, primary_window.width(), ARENA_HEIGHT as f32),
            convert(pos.y as f32, primary_window.height(), ARENA_WIDTH as f32),
            0.0,
        );
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
