use bevy::prelude::*;

#[derive(Component)]
pub struct IsAPlayer;

#[derive(Component)]
pub struct Lives(pub u8);

#[derive(Component)]
pub struct Paddle {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub ui: Option<Entity>
}
