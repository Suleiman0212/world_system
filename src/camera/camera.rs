use super::{movement::CameraMovementPlugin, spawn::CameraSpawnPlugin};
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig {
            motion_speed: 50.0,
            zoom_speed: 0.01,
            min_zoom: 0.01,
            max_zoom: 1.00,
        })
        .add_plugins(CameraSpawnPlugin)
        .add_plugins(CameraMovementPlugin);
    }
}

#[derive(Component)]
pub struct Camera;

#[derive(Resource)]
pub struct CameraConfig {
    pub motion_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}
