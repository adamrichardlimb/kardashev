use crate::world::Mesh;

pub const VOXEL_SIZE: f32 = 0.1;
pub const CHUNK_SIZE: usize = 16;

const CUBE_VERTICES: &[f32] = &[
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0
];

const CUBE_INDICES: &[u32] = &[
    0,1,2,2,3,0, //Back
    4,5,6,6,7,4, //Front
    4,0,3,3,7,4, //Left
    1,5,6,6,2,1, //Right
    3,2,6,6,7,3, //Top
    0,1,5,5,4,0, //Bottom
];

pub type ChunkBlockData = [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]; 

pub struct Chunk {
    //Give me an array of the size of the chunk (x) containing an array of the size of the chunk (y)
    //containing an array the size of the chunk (z) of u8s (block IDs)
    blocks: ChunkBlockData,
    pub mesh: Mesh
}

impl Chunk {
    pub fn new_flat() -> Self {
        let mut blocks = [[[0u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..1 {
                    blocks[x][y][z] = 1; //Call this solid ground for now
                }
            }
        }

        let mesh = generate_mesh(blocks);

        Self {
            blocks,
            mesh
        }
    } 
}

fn generate_mesh(blocks: ChunkBlockData) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
               //If air then just continue
                if blocks[x][y][z] == 0 {
                    continue;
                }

                //Otherwise lets make a mesh for the chunk
                //Right now this is just a terrible implementation as Proof of Concept
                let base_index = (vertices.len() / 3) as u32;
                let cube = unit_cube_vertices((x, y, z));
                vertices.extend_from_slice(&cube);
                indices.extend_from_slice(&unit_cube_indices(base_index));
            }
        }
    }

    Mesh::from_vertices_and_indices(&vertices, &indices)
}

fn unit_cube_vertices(offset: (usize, usize, usize)) -> [f32; 3*8] {
    let mut vertices = [0.0; 24];

    //Every slice is a vertex in the VBO
    for (i, vertex) in CUBE_VERTICES.chunks(3).enumerate() {
        let base = i* 3;
        vertices[base + 0] = (vertex[0] * VOXEL_SIZE/2.0) + (offset.0 as f32 * VOXEL_SIZE);
        vertices[base + 1] = (vertex[1] * VOXEL_SIZE/2.0) + (offset.1 as f32 * VOXEL_SIZE);
        vertices[base + 2] = (vertex[2] * VOXEL_SIZE/2.0) + (offset.2 as f32 * VOXEL_SIZE);
    }

    return vertices;
}

fn unit_cube_indices(base_index: u32) -> [u32; 36] {
    let mut indices = [0u32; 36];

    for (i, &idx) in CUBE_INDICES.iter().enumerate() {
        indices[i] = base_index + idx;
    }

    return indices;
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
