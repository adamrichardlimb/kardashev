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
        let chunk = Chunk::new_flat();
        let chunk2 = Chunk::new_flat();

        chunks.insert((0,0,0), chunk);
        chunks.insert((1,0,0), chunk2);

        Self {
            chunks
        }
    }
}
