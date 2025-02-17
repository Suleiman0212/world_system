use super::map::{MapState, TilesAssets};
use bevy::prelude::*;

pub struct MapAssetsPlugin;
impl Plugin for MapAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, check_is_loaded);
    }
}

fn load_assets(mut tiles_assets: ResMut<TilesAssets>, asset_server: Res<AssetServer>) {
    let water_gltf: Handle<Gltf> = asset_server.load("tiles/water.glb");
    let sand_gltf: Handle<Gltf> = asset_server.load("tiles/sand.glb");
    let dirt_gltf: Handle<Gltf> = asset_server.load("tiles/dirt.glb");
    let stone_gltf: Handle<Gltf> = asset_server.load("tiles/stone.glb");
    let snow_gltf: Handle<Gltf> = asset_server.load("tiles/snow.glb");

    *tiles_assets = TilesAssets {
        water_gltf,
        sand_gltf,
        dirt_gltf,
        stone_gltf,
        snow_gltf,
    }
}

fn check_is_loaded(
    mut map_state: ResMut<MapState>,
    tiles_assets: Res<TilesAssets>,
    asset_server: Res<AssetServer>,
) {
    if map_state.assets_loaded {
        return;
    }
    if !asset_server.is_loaded(&tiles_assets.water_gltf) {
        return;
    }
    if !asset_server.is_loaded(&tiles_assets.sand_gltf) {
        return;
    }
    if !asset_server.is_loaded(&tiles_assets.dirt_gltf) {
        return;
    }
    if !asset_server.is_loaded(&tiles_assets.stone_gltf) {
        return;
    }
    if !asset_server.is_loaded(&tiles_assets.snow_gltf) {
        return;
    }

    map_state.assets_loaded = true;
}
