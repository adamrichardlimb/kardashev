use glam::{Mat4, Vec3};

pub struct Camera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    forward: Vec3,
    up: Vec3
}

pub struct Lens {
    field_of_view_y: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32
}

pub fn get_view_matrix(camera: &Camera) -> Mat4 {
    Mat4::look_at_rh(camera.position, camera.position + camera.forward, camera.up)
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
    pub fn move_by(&mut self, local_delta: Vec3) {
        let right = self.forward.cross(self.up).normalize();
        let up = self.up;
        let forward = self.forward;

        let movement = 
            right * local_delta.x +
            up    * local_delta.y -
            forward * local_delta.z;

        self.position += movement;
    }
 


    pub fn apply_look(&mut self, delta: (f32, f32)) {
        self.yaw += delta.0;
        self.pitch -= delta.1;

        println!("Applying look of {}, {}", delta.0, delta.1);

        //ChatGPT says prevent flip, I assume this is so I cannot look over my own head
        self.pitch = self.pitch.clamp(-1.55, 1.55);

        let dir = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos()
        );

        self.forward = dir.normalize();
    } 

    pub fn new() -> Camera {

        let dir = Vec3::new(
            1.0,
            0.0,
            0.0
        );

        Self {
            position: Vec3::new(0.0, 0.0, 3.0),
            forward: dir.normalize(),
            yaw: 0.0,
            pitch: 0.0,
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
