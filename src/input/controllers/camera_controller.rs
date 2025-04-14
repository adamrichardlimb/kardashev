use std::collections::HashSet;

use glam::Vec3;
use sdl2::keyboard::Keycode;

use crate::input::{Controller, InputAction};

pub struct CameraController {
    pub movement_speed: f32,
    pub look_sensitivity: f32
}

impl<'a> CameraController {
    pub fn new() -> Self {
        Self {
            movement_speed: 0.1,
            look_sensitivity: 0.01
        }
    }
}

impl Controller for CameraController {
    fn map_keys(&mut self, held_keys: HashSet<Keycode>, mouse_motion: Option<(i32, i32)>) -> Vec<InputAction> {
        let mut actions = Vec::new();

        //Handle key presses
        for key in held_keys {
            let delta = match key {
                Keycode::W => Vec3::new(0.0, 0.0, -1.0),
                Keycode::S => Vec3::new(0.0, 0.0, 1.0),
                Keycode::A => Vec3::new(-1.0, 0.0, 0.0),
                Keycode::D => Vec3::new(1.0, 0.0, 0.0),
                Keycode::ESCAPE => return vec!(InputAction::Quit),
                Keycode::F1 => return vec!(InputAction::ToggleDebugModule(1)),
                _ => Vec3::ZERO
            };

            if delta != Vec3::ZERO { actions.push(InputAction::MoveCamera(delta * self.movement_speed)); }
        }

        //Handle mouse motion
        if let Some(mouse_movement) = mouse_motion {
            let adjusted_movement = (mouse_movement.0 as f32 * self.look_sensitivity, mouse_movement.1 as f32 * self.look_sensitivity);
            actions.push(InputAction::LookDelta(adjusted_movement));
        }

        actions
    }
} 
