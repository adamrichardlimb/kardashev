mod chunk;

use std::collections::HashSet;
use std::time::Duration;
use glam::Vec3;
use tracing_subscriber::{reload, EnvFilter};
use crate::Camera;

pub struct DebugOverlay {
    pub filter_handle: reload::Handle<EnvFilter, tracing_subscriber::Registry>,
    pub visible_modules: HashSet<&'static str>,
    pub frame_time_ms: f64,
    pub chunk_count: usize,
    pub draw_calls: usize,
    pub camera_position: Vec3,
    pub camera_pitch: f32,
    pub camera_yaw: f32,
}

impl DebugOverlay {
    pub fn new(filter_handle: reload::Handle<EnvFilter, tracing_subscriber::Registry>) -> Self {
        Self {
            filter_handle,
            visible_modules: HashSet::new(),
            frame_time_ms: 0.0,
            chunk_count: 0,
            draw_calls: 0,
            camera_position: Vec3::ZERO,
            camera_pitch: 0.0,
            camera_yaw: 0.0
        }
    }
 
    pub fn toggle_module(&mut self, module: &'static str) {
        if !self.visible_modules.remove(module) {
            self.visible_modules.insert(module);
        }

        // Rebuild filter string
        let mut filter_str = String::new();
        for m in &self.visible_modules {
            if !filter_str.is_empty() {
                filter_str.push(',');
            }
            filter_str.push_str(&format!("{}=debug", m));
        }

        // Apply to subscriber
        let _ = self.filter_handle.modify(|filter| {
            *filter = EnvFilter::new(filter_str);
        });
    }

    pub fn is_module_visible(&self, module: &str) -> bool {
        self.visible_modules.contains(module)
    }

    pub fn update(&mut self, frame_time: Duration, camera: &Camera) {
        self.frame_time_ms = frame_time.as_secs_f64() * 1000.0;
        self.camera_position = camera.position();
        self.camera_pitch = camera.pitch();
        self.camera_yaw = camera.yaw();
    }
}
