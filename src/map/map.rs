use super::{assets::MapAssetsPlugin, generator::MapGeneratorPlugin, spawn::MapSpawnPlugin};
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapConfig {
            seed: 1,
            noise_scale: 0.05,
            width: 3,
            lenght: 3,
        })
        .init_resource::<TilesAssets>()
        .insert_resource(TileConfig {
            water_threshold: 0.35,
            sand_threshold: 0.38,
            dirt_threshold: 0.6,
            stone_threshold: 0.7,
            snow_threshold: 1.0,
        })
        .insert_resource(MapState {
            assets_loaded: false,
            generated: false,
            tiles_filled: false,
        })
        .insert_resource(TileMap { tiles: vec![] })
        .add_plugins(MapAssetsPlugin)
        .add_plugins(MapGeneratorPlugin)
        .add_plugins(MapSpawnPlugin);
    }
}

#[allow(unused)]
#[derive(Component, Clone, Copy, Debug)]
pub enum TileType {
    Water,
    Sand,
    Dirt,
    Stone,
    Snow,
}

#[allow(unused)]
#[derive(Component, Clone, Copy, Debug)]
pub struct Tile {
    pub x: u32,
    pub z: u32,
    pub tile_type: TileType,
}

#[allow(unused)]
#[derive(Resource, Default)]
pub struct TilesAssets {
    pub water_gltf: Handle<Gltf>,
    pub sand_gltf: Handle<Gltf>,
    pub dirt_gltf: Handle<Gltf>,
    pub stone_gltf: Handle<Gltf>,
    pub snow_gltf: Handle<Gltf>,
}

#[allow(unused)]
#[derive(Resource)]
pub struct TileMap {
    pub tiles: Vec<Tile>,
}

#[allow(unused)]
#[derive(Resource)]
pub struct MapConfig {
    pub seed: u32,
    pub noise_scale: f64,
    pub width: u32,
    pub lenght: u32,
}

#[allow(unused)]
#[derive(Resource)]
pub struct TileConfig {
    pub water_threshold: f64,
    pub sand_threshold: f64,
    pub dirt_threshold: f64,
    pub stone_threshold: f64,
    pub snow_threshold: f64,
}

#[allow(unused)]
#[derive(Resource)]
pub struct MapState {
    pub assets_loaded: bool,
    pub generated: bool,
    pub tiles_filled: bool,
}
