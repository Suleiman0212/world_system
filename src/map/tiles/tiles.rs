use super::materials::TileMaterialsPlugin;
use bevy::prelude::*;

pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TileMaterialsPlugin);
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
