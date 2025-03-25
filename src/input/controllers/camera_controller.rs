use glam::Vec3;
use sdl2::keyboard::Keycode;

use crate::input::{Controller, InputAction};

pub struct CameraController {
    pub movement_speed: f32
}

impl<'a> CameraController {
    pub fn new() -> Self {
        Self {
            movement_speed: 0.1
        }
    }
}

impl Controller for CameraController {
    fn handle_key(&mut self, key: Keycode) -> Option<InputAction> {
        let delta = match key {
            Keycode::W => Vec3::new(0.0, 0.0, -1.0),
            Keycode::S => Vec3::new(0.0, 0.0, 1.0),
            Keycode::A => Vec3::new(-1.0, 0.0, 0.0),
            Keycode::D => Vec3::new(1.0, 0.0, 0.0),
            Keycode::ESCAPE => return Some(InputAction::Quit),
            _ => Vec3::ZERO
        };

        if delta != Vec3::ZERO {
            return Some(InputAction::MoveCamera(delta * self.movement_speed));
        } else {
            return None;
        }
    }
} 
