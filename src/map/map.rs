use super::{
    generator::MapGeneratorPlugin, spawn::MapSpawnPlugin, tiles::tiles::TileMap,
    tiles::tiles::TilesPlugin,
};
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapConfig {
            seed: 1,
            map_scale: 3.0,
            noise_scale: 0.05,
            width: 52,
            lenght: 52,
        })
        .insert_resource(MapState {
            generated: false,
            tiles_filled: false,
            entitys_spawned: false,
        })
        .insert_resource(TileMap { tiles: vec![] })
        .add_plugins(MapGeneratorPlugin)
        .add_plugins(TilesPlugin)
        .add_plugins(MapSpawnPlugin);
    }
}

#[allow(unused)]
#[derive(Resource)]
pub struct MapConfig {
    pub seed: u32,
    pub map_scale: f32,
    pub noise_scale: f64,
    pub width: u32,
    pub lenght: u32,
}

#[allow(unused)]
#[derive(Resource)]
pub struct MapState {
    pub generated: bool,
    pub tiles_filled: bool,
    pub entitys_spawned: bool,
}
