use super::camera::Camera;
use bevy::prelude::*;

pub struct CameraSpawnPlugin;
impl Plugin for CameraSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .insert_resource(AmbientLight {
                brightness: 500.0,
                ..default()
            });
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scale: 0.05,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(Vec3::new(10.0, 15.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 3000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            300.0_f32.to_radians(),
            0.0,
            0.0,
        )),
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1800.0,
    });
}
