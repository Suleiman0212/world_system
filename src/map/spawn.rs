use super::map::{MapState, Tile, TileMap, TileType, TilesAssets};
use bevy::prelude::*;

pub struct MapSpawnPlugin;
impl Plugin for MapSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_tiles);
    }
}

fn spawn_tiles(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    tiles_assets: Res<TilesAssets>,
    tile_map: Res<TileMap>,
    gltf: Res<Assets<Gltf>>,
    tiles_query: Query<(&Name, Entity), With<Tile>>,
) {
    if map_state.tiles_filled {
        return;
    }

    if !map_state.assets_loaded {
        return;
    }

    if !map_state.generated {
        return;
    }

    for (name, tile_entity) in tiles_query.iter() {
        println!("Removing: {}", name);
        commands.entity(tile_entity).despawn_recursive();
    }

    let water_tile = &gltf.get(&tiles_assets.water_gltf).unwrap().scenes[0];
    let sand_tile = &gltf.get(&tiles_assets.sand_gltf).unwrap().scenes[0];
    let dirt_tile = &gltf.get(&tiles_assets.dirt_gltf).unwrap().scenes[0];
    let stone_tile = &gltf.get(&tiles_assets.stone_gltf).unwrap().scenes[0];
    let snow_tile = &gltf.get(&tiles_assets.snow_gltf).unwrap().scenes[0];

    for tile in tile_map.tiles.iter() {
        let y_pos = match tile.tile_type {
            TileType::Water => 0.0,
            TileType::Sand => 1.0,
            TileType::Dirt => 2.0,
            TileType::Stone => 3.0,
            TileType::Snow => 4.0,
        };
        commands.spawn((
            Name::new(format!("{:?}: X{}.Y{}", tile.tile_type, tile.x, tile.z)),
            SceneRoot(match tile.tile_type {
                TileType::Water => water_tile.clone_weak(),
                TileType::Sand => sand_tile.clone_weak(),
                TileType::Dirt => dirt_tile.clone_weak(),
                TileType::Stone => stone_tile.clone_weak(),
                TileType::Snow => snow_tile.clone_weak(),
            }),
            Transform::from_translation(Vec3::new(tile.x as f32 * 2.0, y_pos, tile.z as f32 * 2.0)),
            GlobalTransform::default(),
            tile.tile_type,
            tile.clone(),
        ));
    }

    map_state.tiles_filled = true;
}
