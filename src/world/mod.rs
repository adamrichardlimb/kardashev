pub mod chunk;

use std::collections::HashMap;
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

        for chunk_x in -1..=1 {
            for chunk_z in -1..=1 {
                let chunk_pos = (chunk_x, 0, chunk_z);
                let chunk = Chunk::new_flat();
                
                chunks.insert(chunk_pos, chunk);
            }
        }

        Self {
            chunks
        }
    }
}
