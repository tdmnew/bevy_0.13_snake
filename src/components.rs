use bevy::prelude::*;

/*
 *
 * Components
 *
 */
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}


#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
    pub movement_timer: Timer,
}
impl Default for SnakeHead {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
            movement_timer: Timer::from_seconds(0.20, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component, PartialEq, Eq)]
pub struct Food;
