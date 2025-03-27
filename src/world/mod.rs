pub mod chunk;

use std::collections::HashMap;
use noise::Perlin;

use crate::rendering::mesh::Mesh;
use crate::world::chunk::Chunk;

pub type ChunkPos = (i32, i32, i32);
pub type ChunkMap = HashMap<ChunkPos, Chunk>;

pub struct World {
    pub chunks: ChunkMap
}

impl World {
    pub fn new() -> Self {
        let mut chunks = ChunkMap::new();

        let perlin = Perlin::new(24601);

        let chunk = Chunk::from_perlin_noise((0,0,0), &perlin);
        let chunk2 = Chunk::from_perlin_noise((1,0,0), &perlin);
        let chunk3 = Chunk::from_perlin_noise((1,1,0), &perlin);

        chunks.insert((0,0,0), chunk);
        chunks.insert((1,0,0), chunk2);
        chunks.insert((1,1,0), chunk3);

        Self {
            chunks
        }
    }
}
