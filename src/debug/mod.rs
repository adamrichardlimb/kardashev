mod chunk;

use std::time::Duration;
use glam::Vec3;
use crate::{rendering::RenderCommand, Camera};
use crate::rendering::text::{self};
use sdl2::{ttf::Font, pixels::Color};


pub struct DebugOverlay {
    pub frame_time_ms: f64,
    pub chunk_count: usize,
    pub draw_calls: usize,
    pub camera_position: Vec3,
    pub camera_pitch: f32,
    pub camera_yaw: f32,
}

impl DebugOverlay {
    pub fn new() -> Self {
        Self {
            frame_time_ms: 0.0,
            chunk_count: 0,
            draw_calls: 0,
            camera_position: Vec3::ZERO,
            camera_pitch: 0.0,
            camera_yaw: 0.0
        }
    }

    pub fn update(&mut self, frame_time: Duration, chunk_count: usize, camera: &Camera) {
        self.frame_time_ms = frame_time.as_secs_f64() * 1000.0;
        self.chunk_count = chunk_count;
        self.camera_position = camera.position();
        self.camera_pitch = camera.pitch();
        self.camera_yaw = camera.yaw();
    }

    
}

