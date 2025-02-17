use super::camera::CameraConfig;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct CameraMovementPlugin;
impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_by_input, scroll_events));
    }
}

fn move_by_input(
    mut query: Query<&mut Transform, With<Camera3d>>,
    camera_config: Res<CameraConfig>,
    button: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if button.pressed(KeyCode::KeyW) {
        direction += Vec3::new(-1.0, 0.0, -1.0);
    }
    if button.pressed(KeyCode::KeyS) {
        direction += Vec3::new(1.0, 0.0, 1.0);
    }
    if button.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, -1.0);
    }
    if button.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 1.0);
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let mut camera_transform = match query.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };

    camera_transform.translation += direction * camera_config.motion_speed * time.delta_secs();
}

fn scroll_events(
    mut evr_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera3d>>,
    camera_config: Res<CameraConfig>,
) {
    let mut projection = match query.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    if let Projection::Orthographic(ref mut ortho) = *projection {
        for ev in evr_scroll.read() {
            let zoom_change = -ev.y * camera_config.zoom_speed;

            ortho.scale =
                (ortho.scale + zoom_change).clamp(camera_config.min_zoom, camera_config.max_zoom);
        }
    }
}
