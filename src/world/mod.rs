pub mod chunk;
pub mod chunk_mesh_manager;


use crate::rendering::render_context::RenderMesh;
use crate::world::chunk::Chunk;
use crate::{VOXEL_SIZE, CHUNK_SIZE};
use chunk_mesh_manager::{generate_mesh, model_for_chunk, ChunkMeshManager};
use noise::Perlin;
use std::collections::{HashMap, HashSet};
use glam::{Vec3, Mat4};

pub enum ChunkChange {
    Loaded(ChunkPos),
    Unloaded(ChunkPos),
}

pub type ChunkPos = (i32, i32, i32);
pub type ChunkMap = HashMap<ChunkPos, Chunk>;
const CHUNK_DISTANCE: i32 = 5;

pub struct World {
    pub seed: u32,
    pub chunks: ChunkMap
}

pub fn world_to_chunk_pos(pos: Vec3) -> ChunkPos {
    let size = CHUNK_SIZE as f32 * VOXEL_SIZE;
    (
        (pos.x / size).floor() as i32,
        (pos.y / size).floor() as i32,
        (pos.z / size).floor() as i32,
    )
}

//Every chunk is 16x16
pub fn chunk_range(center: (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    let (cx, cy, cz) = center;

    (cx - CHUNK_DISTANCE..=cx + CHUNK_DISTANCE)
        .flat_map(move |x| {
            (cy - CHUNK_DISTANCE..=cy + CHUNK_DISTANCE).flat_map(move |y| {
                (cz - CHUNK_DISTANCE..=cz + CHUNK_DISTANCE).map(move |z| (x, y, z))
            })
        })
}

impl World {
    pub fn new() -> Self {
        Self {
            seed: 24601,
            chunks: ChunkMap::new()
        }
    }

    pub fn update(&mut self, player_pos: Vec3) -> Vec<ChunkChange> {
        let mut changes = Vec::new();

       //Generate chunks near the player based on the seed
       let center = world_to_chunk_pos(player_pos);
       let loaded_chunks: HashSet<ChunkPos> = chunk_range(center).collect();
       let perlin = Perlin::new(self.seed);

       for &pos in &loaded_chunks {
           if !self.chunks.contains_key(&pos) {
               self.chunks.entry(pos).or_insert_with(|| Chunk::from_perlin_noise(pos, &perlin));
               changes.push(ChunkChange::Loaded(pos));
           }
       }

       self.chunks.retain(|&pos, _| {
            if loaded_chunks.contains(&pos) {
                true
            } else {
                changes.push(ChunkChange::Unloaded(pos));
                false
            }
        });

        changes
    }
}
