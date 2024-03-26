use bevy::prelude::*;

use crate::u7f8au7fbd::startup::Player;

pub fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.z -= 0.05;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.z += 0.05;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 0.05;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += 0.05;
        }
    }
}
