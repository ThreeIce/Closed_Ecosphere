use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::config::Config;

pub fn camera_control(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut evr_scroll: EventReader<MouseWheel>,
    config: Res<Config>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
) {
    let (mut transform, mut projection) = query.get_single_mut().unwrap();
    let speed = config.camera_speed;
    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation.y += speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation.y -= speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += speed * time.delta_secs();
    }
    for ev in evr_scroll.read() {
        projection.scale *= 1.0 - ev.y * config.camera_zoom_speed;
    }
}