use crate::player::*;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn spawn_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut player1 = PlayerBundle::default();
    let player1_ui = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            Transform::default(),
        ))
        .id();
    player1.paddle.ui = Some(player1_ui);


    let mut player2 = PlayerBundle::default();
    let player2_ui = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
            Transform::default(),
        ))
        .id();
    player2.paddle.ui = Some(player2_ui);

    commands.spawn_batch([
        player1, player2 
    ]);   
}
