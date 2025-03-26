mod chunk;

use crate::rendering::mesh::Mesh;
use crate::world::chunk::Chunk;

pub struct World {
    pub meshes: Vec<Mesh>,
}

impl World {
    pub fn new() -> Self {
        let chunk = Chunk::new_flat();
        let mut meshes = Vec::new();
        meshes.push(chunk.generate_mesh());
        Self {
            meshes
        }
    }
}
