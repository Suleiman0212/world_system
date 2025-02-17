use bevy::prelude::*;
use camera::camera::CameraPlugin;
use map::map::MapPlugin;
use ui::ui::UIPlugin;

mod camera;
mod map;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .run();
}
