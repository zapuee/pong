use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player_mark: IsAPlayer,
    pub paddle: Paddle,
    pub lives: Lives,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player_mark: IsAPlayer,
            paddle: Paddle {
                move_up: KeyCode::KeyU,
                move_down: KeyCode::KeyD,
                ui: None
            },
            lives: Lives(3),
        }
    }
}


