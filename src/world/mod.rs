use crate::rendering::mesh::Mesh;

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

pub struct World {
    pub meshes: Vec<Mesh>,
}

impl World {
    pub fn new() -> Self {
        let mut meshes = Vec::new();
        meshes.push(Mesh::from_vertices_and_indices(CUBE_VERTICES, CUBE_INDICES));
        Self {
            meshes
        }
    }
}
