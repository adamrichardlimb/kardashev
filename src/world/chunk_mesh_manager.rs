use crate::events::{Event, EventHandler, EventType};
use crate::world::{ChunkPos, chunk::ChunkBlockData};
use crate::rendering::mesh::Mesh;
use crate::{VOXEL_SIZE, CHUNK_SIZE};
use std::collections::{HashMap, HashSet};
use glam::{Mat4, Vec3};
use crate::rendering::render_context::RenderMesh;

pub struct ChunkMeshManager {
    meshes: HashMap<ChunkPos, RenderMesh>,
}

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

impl ChunkMeshManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new()
        }
    }

    pub fn get_or_create(&mut self, pos: ChunkPos, blocks: &ChunkBlockData) {
        self.meshes.entry(pos).or_insert_with(|| generate_mesh(pos, blocks));
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ChunkPos, &RenderMesh)> {
        self.meshes.iter()
    }

    pub fn meshes(&self) -> Vec<&RenderMesh> {
        let mut meshes = Vec::new();
        for (_pos, mesh) in self.iter() {
            meshes.push(mesh);
        }
        meshes
    }
}

impl EventHandler for ChunkMeshManager {
    fn on_event(&mut self, event: &Event) {
        if let Event::ChunkLoaded(pos, blocks) = event {
            self.get_or_create(*pos, blocks);
        } else if let Event::ChunkUnloaded(pos) = event {
            self.meshes.remove(pos);
        }
    }

    fn event_types(&self) -> Vec<EventType> {
        let mut events = Vec::new();
        events.push(EventType::ChunkLoaded);
        events.push(EventType::ChunkUnloaded);
        events
    }
}

fn generate_mesh(pos: ChunkPos, blocks: &ChunkBlockData) -> RenderMesh {
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

    RenderMesh{
        model: Mat4::from_translation(
            Vec3::new(
                pos.0 as f32,
                pos.1 as f32,
                pos.2 as f32
            ) * CHUNK_SIZE as f32 * VOXEL_SIZE
        ),
        mesh: Mesh::from_vertices_and_indices(&vertices, &indices)
    }
}

pub fn model_for_chunk(pos: ChunkPos) -> Mat4 {
    Mat4::from_translation(
        Vec3::new(
            pos.0 as f32,
            pos.1 as f32,
            pos.2 as f32
        ) * CHUNK_SIZE as f32 * VOXEL_SIZE
    )
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
