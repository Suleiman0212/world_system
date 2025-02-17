use super::{generator::MapGeneratorPlugin, spawn::MapSpawnPlugin};
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapConfig {
            seed: 1,
            map_scale: 1.0,
            noise_scale: 0.05,
            width: 3,
            lenght: 3,
        })
        .insert_resource(MapState {
            generated: false,
            tiles_filled: false,
            entitys_spawned: false,
        })
        .insert_resource(TileMap { tiles: vec![] })
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

impl TileType {
    pub const WATER_TILE_POS_Y: f32 = 0.0;
    pub const SAND_TILE_POS_Y: f32 = 1.0;
    pub const DIRT_TILE_POS_Y: f32 = 2.0;
    pub const STONE_TILE_POS_Y: f32 = 3.0;
    pub const SNOW_TILE_POS_Y: f32 = 4.0;

    pub fn get_pos_y(&self) -> f32 {
        match self {
            Self::Water => Self::WATER_TILE_POS_Y,
            Self::Sand => Self::SAND_TILE_POS_Y,
            Self::Dirt => Self::DIRT_TILE_POS_Y,
            Self::Stone => Self::STONE_TILE_POS_Y,
            Self::Snow => Self::SNOW_TILE_POS_Y,
        }
    }

    pub const WATER_TILE_COLOR: Color = Color::srgb(47.0 / 255.0, 106.0 / 255.0, 229.0 / 255.0);
    pub const SAND_TILE_COLOR: Color = Color::srgb(229.0 / 255.0, 180.0 / 255.0, 47.0 / 255.0);
    pub const DIRT_TILE_COLOR: Color = Color::srgb(129.0 / 255.0, 229.0 / 255.0, 47.0 / 255.0);
    pub const STONE_TILE_COLOR: Color = Color::srgb(117.0 / 255.0, 124.0 / 255.0, 124.0 / 255.0);
    pub const SNOW_TILE_COLOR: Color = Color::srgb(230.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0);

    pub fn get_color(&self) -> Color {
        match self {
            Self::Water => Self::WATER_TILE_COLOR,
            Self::Sand => Self::SAND_TILE_COLOR,
            Self::Dirt => Self::DIRT_TILE_COLOR,
            Self::Stone => Self::STONE_TILE_COLOR,
            Self::Snow => Self::SNOW_TILE_COLOR,
        }
    }

    pub const WATER_TILE_THRESHOLD: f64 = 0.35;
    pub const SAND_TILE_THRESHOLD: f64 = 0.38;
    pub const DIRT_TILE_THRESHOLD: f64 = 0.6;
    pub const STONE_TILE_THRESHOLD: f64 = 0.7;
    pub const SNOW_TILE_THRESHOLD: f64 = 1.0;

    pub fn get_treshhold(&self) -> f64 {
        match self {
            Self::Water => Self::WATER_TILE_THRESHOLD,
            Self::Sand => Self::SAND_TILE_THRESHOLD,
            Self::Dirt => Self::DIRT_TILE_THRESHOLD,
            Self::Stone => Self::STONE_TILE_THRESHOLD,
            Self::Snow => Self::SNOW_TILE_THRESHOLD,
        }
    }
}

#[allow(unused)]
#[derive(Component, Clone, Copy, Debug)]
pub struct Tile {
    pub x: u32,
    pub z: u32,
    pub tile_type: TileType,
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
