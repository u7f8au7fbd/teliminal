use bevy::prelude::*;

use crate::u7f8au7fbd::startup::DebugText;
use crate::u7f8au7fbd::startup::Player;

pub fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let speed = 0.05;
    for mut transform in query.iter_mut() {
        let rotation = transform.rotation;
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation -= rotation * Vec3::Z * speed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += rotation * Vec3::Z * speed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation -= rotation * Vec3::X * speed;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation += rotation * Vec3::X * speed;
        }
    }
}

pub fn move_camera(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyE) {
            transform.rotation *= Quat::from_rotation_y(-0.005);
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            transform.rotation *= Quat::from_rotation_y(0.005);
        }
        if keyboard_input.pressed(KeyCode::Space) {
            let rotation = transform.rotation.y;
            println!("Cos: {:?}", rotation.cos());
            println!("Sin: {:?}", rotation.sin());
        }
    }
}

pub fn debug_update(
    mut text_query: Query<&mut Text, With<DebugText>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player in player_query.iter() {
        for mut text in text_query.iter_mut() {
            let rotation = player.rotation.y;
            text.sections[0].value = format!("Sin:{:.3},Cos:{:.3}", rotation.sin(), rotation.cos());
        }
    }
}
