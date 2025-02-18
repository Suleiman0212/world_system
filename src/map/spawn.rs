use super::map::{MapConfig, MapState, Tile, TileMap, TileType};
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
    map_config: Res<MapConfig>,
    tile_map: Res<TileMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tiles_query: Query<(&Name, Entity), With<Tile>>,
) {
    if map_state.tiles_filled {
        return;
    }

    if !map_state.generated {
        return;
    }

    for (name, tile_entity) in tiles_query.iter() {
        println!("Removing: {}", name);
        commands.entity(tile_entity).despawn_recursive();
    }

    let cube_mesh = meshes.add(Cuboid::new(
        1.0 * map_config.map_scale,
        1.0 * map_config.map_scale,
        1.0 * map_config.map_scale,
    ));

    let water_material = materials.add(StandardMaterial {
        base_color: TileType::Water.get_color(),
        ..default()
    });
    let sand_material = materials.add(StandardMaterial {
        base_color: TileType::Sand.get_color(),
        ..default()
    });
    let dirt_material = materials.add(StandardMaterial {
        base_color: TileType::Dirt.get_color(),
        ..default()
    });
    let stone_material = materials.add(StandardMaterial {
        base_color: TileType::Stone.get_color(),
        ..default()
    });
    let snow_material = materials.add(StandardMaterial {
        base_color: TileType::Snow.get_color(),
        ..default()
    });

    for tile in tile_map.tiles.iter() {
        commands.spawn((
            Name::new(format!("{:?}: X{}.Y{}", tile.tile_type, tile.x, tile.z)),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(match tile.tile_type {
                TileType::Water => water_material.clone(),
                TileType::Sand => sand_material.clone(),
                TileType::Dirt => dirt_material.clone(),
                TileType::Stone => stone_material.clone(),
                TileType::Snow => snow_material.clone(),
            }),
            Transform::from_translation(Vec3::new(
                tile.x as f32 * map_config.map_scale,
                tile.tile_type.get_pos_y(),
                tile.z as f32 * map_config.map_scale,
            )),
            tile.clone(),
        ));
    }

    map_state.tiles_filled = true;
}
