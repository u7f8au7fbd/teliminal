use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;

use crate::u7f8au7fbd::startup::DebugText;
use crate::u7f8au7fbd::startup::Player;

pub fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 16.;
    for mut transform in query.iter_mut() {
        let rotation = transform.rotation;
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation -= rotation * Vec3::Z * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += rotation * Vec3::Z * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation -= rotation * Vec3::X * speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation += rotation * Vec3::X * speed * time.delta_seconds();
        }
    }
}

pub fn move_camera(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let sensitivity = 0.005;
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotation *= Quat::from_rotation_y(-sensitivity * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotation *= Quat::from_rotation_y(sensitivity * time.delta_seconds());
        }
    }
}

pub fn debug_update(mut text_query: Query<&mut Text, With<DebugText>>) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = "Debug".to_string();
    }
}
