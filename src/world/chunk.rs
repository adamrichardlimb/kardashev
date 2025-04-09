use noise::{Perlin, NoiseFn};

use super::ChunkPos;

pub const VOXEL_SIZE: f32 = 0.1;
pub const CHUNK_SIZE: usize = 16;


pub type ChunkBlockData = [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]; 

pub struct Chunk {
    //Give me an array of the size of the chunk (x) containing an array of the size of the chunk (y)
    //containing an array the size of the chunk (z) of u8s (block IDs)
    pub blocks: ChunkBlockData
}

impl Chunk {
    pub fn from_perlin_noise((chunk_x, _chunk_y, chunk_z): (i32, i32, i32), perlin: &Perlin) -> Self {
        let mut blocks = [[[0u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        let world_x = chunk_x * CHUNK_SIZE as i32;
        let world_z = chunk_z * CHUNK_SIZE as i32;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let nx = (world_x + x as i32) as f64 * 0.05;
                let nz = (world_z + z as i32) as f64 * 0.04;

                let height = ((perlin.get([nx, nz]) + 1.0) * 0.5 * CHUNK_SIZE as f64 * 0.5) as usize;

                for y in 0..CHUNK_SIZE {
                    if y <= height {
                        blocks[x][y][z] = 1;
                    }
                }
            }
        }

        Self {
            blocks
        }
    }

    pub fn new_flat() -> Self {
        let mut blocks = [[[0u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..1 {
                    blocks[x][y][z] = 1; //Call this solid ground for now
                }
            }
        }

        Self {
            blocks
        }
    }

    pub fn blocks(&self) -> &ChunkBlockData {
        &self.blocks
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = ((usize, usize, usize), u8);
    type IntoIter = ChunkIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIter {chunk: self, x: 0, y: 0, z: 0}
    }
}

pub struct ChunkIter<'a> {
    chunk: &'a Chunk,
    x: usize,
    y: usize,
    z: usize
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = ((usize, usize, usize), u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= CHUNK_SIZE {
            return None;
        }

        let pos = (self.x, self.y, self.z);
        let value = self.chunk.blocks[self.x][self.y][self.z];

        self.x += 1;
        if self.x >= CHUNK_SIZE {
            self.x = 0;
            self.z += 1;
            if self.z >= CHUNK_SIZE {
                self.z = 0;
                self.y += 1;
            }
        }

        Some((pos, value))
    }
}
