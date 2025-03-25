use glam::{Mat4, Vec3};

pub struct Camera {
    position: Vec3,
    center: Vec3,
    up: Vec3
}

pub struct Lens {
    field_of_view_y: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32
}

pub fn get_view_matrix(camera: &Camera) -> Mat4 {
    Mat4::look_at_rh(camera.position, camera.center, camera.up)
}

pub fn get_projection_matrix(lens: &Lens) -> Mat4 {
    Mat4::perspective_rh_gl(
        lens.field_of_view_y,
        lens.aspect_ratio,
        lens.z_near,
        lens.z_far
    )
}

impl Camera {
    pub fn move_by(&mut self, delta: Vec3) {
        self.position += delta;
        self.center += delta;
    }

    pub fn new() -> Camera {
        Self {
            position: Vec3::new(0.0, 0.0, 3.0),
            center: Vec3::ZERO,
            up: Vec3::Y
        }
    }
}

impl Lens {
    pub fn new() -> Lens {
        Self {
            field_of_view_y: std::f32::consts::FRAC_PI_3,
            aspect_ratio: 800 as f32 / 600 as f32,
            z_near: 0.1,
            z_far: 100.0
        }
    }
}
