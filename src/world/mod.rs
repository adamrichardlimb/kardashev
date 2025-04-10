pub mod chunk;
pub mod chunk_mesh_manager;

use std::collections::hash_map::Entry::Vacant;
use crate::events::{EventQueue, Event, Event::ChunkUnloaded};
use crate::world::chunk::Chunk;
use crate::{VOXEL_SIZE, CHUNK_SIZE};
use noise::Perlin;
use std::collections::{HashMap, HashSet};
use glam::Vec3;


pub type ChunkPos = (i32, i32, i32);
pub type ChunkMap = HashMap<ChunkPos, Chunk>;
const CHUNK_DISTANCE: i32 = 3;

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

    pub fn update(&mut self, player_pos: Vec3, event_queue: &mut EventQueue) {
       //Generate chunks near the player based on the seed
       let center = world_to_chunk_pos(player_pos);
       let loaded_chunks: HashSet<ChunkPos> = chunk_range(center).collect();
       let perlin = Perlin::new(self.seed);

       for &pos in &loaded_chunks {
           if let Vacant(entry) = self.chunks.entry(pos) {
               let chunk = Chunk::from_perlin_noise(pos, &perlin);
               let blocks = chunk.blocks.clone();
               entry.insert(chunk);
               event_queue.push_event(Event::ChunkLoaded(pos, blocks));
           }
       }

       self.chunks.retain(|&pos, _| {
            if loaded_chunks.contains(&pos) {
                true
            } else {
                event_queue.push_event(ChunkUnloaded(pos));
                false
            }
        });

    }
}
