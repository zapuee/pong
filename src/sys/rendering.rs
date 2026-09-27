use bevy::prelude::*;
use crate::components::*;

pub fn render_paddles(
    paddles: Query<&Paddle>,
    mut transforms: Query<&mut Transform>
) {
    for paddle in &paddles {
        if let Some(ui) = paddle.ui
    }
}
