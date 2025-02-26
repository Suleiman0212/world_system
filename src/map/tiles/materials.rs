use super::tiles::TileType;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct TileMaterialsPlugin;
impl Plugin for TileMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WaterMaterial>::default());
    }
}

const WATER_SHADER_ASSET_PATH: &str = "shaders/water.wgsl";
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WaterMaterial {}
impl Material for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        WATER_SHADER_ASSET_PATH.into()
    }
}

pub enum TileMaterial {
    Standart(StandardMaterial),
    Water(WaterMaterial),
}

impl TileMaterial {
    //pub const WATER_TILE_COLOR: Color = Color::srgb(47.0 / 255.0, 180.0 / 255.0, 229.0 / 255.0);
    pub const SAND_TILE_COLOR: Color = Color::srgb(229.0 / 255.0, 180.0 / 255.0, 47.0 / 255.0);
    pub const DIRT_TILE_COLOR: Color = Color::srgb(47.0 / 255.0, 229.0 / 255.0, 47.0 / 255.0);
    pub const STONE_TILE_COLOR: Color = Color::srgb(117.0 / 255.0, 124.0 / 255.0, 124.0 / 255.0);
    pub const SNOW_TILE_COLOR: Color = Color::srgb(230.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0);

    pub fn from_tile_type(tile_type: TileType) -> Self {
        match tile_type {
            TileType::Water => Self::Water(WaterMaterial {}),
            TileType::Sand => Self::Standart(StandardMaterial {
                base_color: Self::SAND_TILE_COLOR,
                ..default()
            }),
            TileType::Dirt => Self::Standart(StandardMaterial {
                base_color: Self::DIRT_TILE_COLOR,
                ..default()
            }),
            TileType::Stone => Self::Standart(StandardMaterial {
                base_color: Self::STONE_TILE_COLOR,
                ..default()
            }),
            TileType::Snow => Self::Standart(StandardMaterial {
                base_color: Self::SNOW_TILE_COLOR,
                ..default()
            }),
        }
    }
}
