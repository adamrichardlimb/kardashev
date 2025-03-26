use crate::world::Mesh;

pub const CHUNK_SIZE: usize = 16;

const CUBE_VERTICES: &[f32] = &[
    -0.5, -0.5, -0.5, //Back, bottom left
     0.5, -0.5, -0.5, //Back, bottom right
     0.5,  0.5, -0.5, //Back, top right
    -0.5,  0.5, -0.5, //Back, top left
    -0.5, -0.5,  0.5, //Front, bottom left
     0.5, -0.5,  0.5, //Front, bottom right
     0.5,  0.5,  0.5, //Front, top right
    -0.5,  0.5,  0.5  //Front, top left
];

const CUBE_INDICES: &[u32] = &[
    0,1,2,2,3,0, //Back
    4,5,6,6,7,4, //Front
    4,0,3,3,7,4, //Left
    1,5,6,6,2,1, //Right
    3,2,6,6,7,3, //Top
    0,1,5,5,4,0, //Bottom
];

pub struct Chunk {
    //Give me an array of the size of the chunk (x) containing an array of the size of the chunk (y)
    //containing an array the size of the chunk (z) of u8s (block IDs)
    blocks: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
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

        Self {blocks}
    }

    pub fn generate_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    //If air then just continue
                    if self.blocks[x][y][z] == 0 {
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
}

fn unit_cube_vertices(offset: (usize, usize, usize)) -> [f32; 3*8] {
    let mut vertices = [0.0; 24];

    //Every slice is a vertex in the VBO
    for (i, vertex) in CUBE_VERTICES.chunks(3).enumerate() {
        let base = i* 3;
        vertices[base + 0] = vertex[0] + offset.0 as f32;
        vertices[base + 1] = vertex[1] + offset.1 as f32;
        vertices[base + 2] = vertex[2] + offset.2 as f32;
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
