use super::map::{MapConfig, MapState};
use super::tiles::materials::{TileMaterial, WaterMaterial};
use super::tiles::tiles::{Tile, TileMap};
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
    mut materials: (
        ResMut<Assets<StandardMaterial>>,
        ResMut<Assets<WaterMaterial>>,
    ),
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

    for tile in tile_map.tiles.iter() {
        let material_handle = TileMaterial::from_tile_type(tile.tile_type);
        match material_handle {
            TileMaterial::Water(m) => {
                commands.spawn((
                    Name::new(format!("{:?}: X{}.Y{}", tile.tile_type, tile.x, tile.z)),
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(materials.1.add(m)),
                    Transform::from_translation(Vec3::new(
                        tile.x as f32 * map_config.map_scale,
                        tile.tile_type.get_pos_y() * map_config.map_scale,
                        tile.z as f32 * map_config.map_scale,
                    )),
                    tile.clone(),
                ));
            }
            TileMaterial::Standart(m) => {
                commands.spawn((
                    Name::new(format!("{:?}: X{}.Y{}", tile.tile_type, tile.x, tile.z)),
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(materials.0.add(m)),
                    Transform::from_translation(Vec3::new(
                        tile.x as f32 * map_config.map_scale,
                        tile.tile_type.get_pos_y() * map_config.map_scale,
                        tile.z as f32 * map_config.map_scale,
                    )),
                    tile.clone(),
                ));
            }
        }
    }

    map_state.tiles_filled = true;
}
