use glam::Mat4;
use crate::rendering::Camera;
use crate::rendering::Mesh;
use crate::rendering::text::Surface2D;

pub struct RenderMesh<'frame> {
    pub mesh: &'frame Mesh,
    pub model: Mat4
}

pub struct RenderContext<'frame> {
    pub camera: &'frame Camera,
    pub meshes: Vec<RenderMesh<'frame>>,
    pub quads: Vec<&'frame Surface2D>
}
