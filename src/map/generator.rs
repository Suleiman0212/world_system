use super::map::{MapConfig, MapState, Tile, TileConfig, TileMap, TileType};
use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

pub struct MapGeneratorPlugin;
impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_map_by_noise);
    }
}

fn generate_map_by_noise(
    map_config: Res<MapConfig>,
    tile_config: Res<TileConfig>,
    mut tile_map: ResMut<TileMap>,
    mut map_state: ResMut<MapState>,
    tiles_query: Query<Entity, With<Tile>>,
    mut commands: Commands,
) {
    if map_state.generated {
        return;
    }

    for tile_entity in tiles_query.iter() {
        tile_map.tiles = vec![];
        commands.entity(tile_entity).despawn_recursive();
    }

    let simplex = OpenSimplex::new(map_config.seed);

    for x in 0..map_config.lenght {
        for z in 0..map_config.width {
            let height = simplex.get([
                x as f64 * map_config.noise_scale,
                z as f64 * map_config.noise_scale,
            ]);
            let height = (height + 1.0) / 2.0;

            let tile_type = if height < tile_config.water_threshold {
                TileType::Water
            } else if height < tile_config.sand_threshold {
                TileType::Sand
            } else if height < tile_config.dirt_threshold {
                TileType::Dirt
            } else if height < tile_config.stone_threshold {
                TileType::Stone
            } else {
                TileType::Snow
            };

            let tile = Tile { x, z, tile_type };
            tile_map.tiles.push(tile);
        }
    }

    map_state.generated = true;
}
