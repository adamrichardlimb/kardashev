use std::collections::HashMap;
use glam::Vec3;
use crate::input::controllers::MouseMotion;
use sdl2::keyboard::Keycode;
use crate::input::{Controller, InputAction};
use super::{Input, KeyMap};

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
    fn keymap(&self) -> KeyMap {
        let mut keymap = HashMap::new();
        keymap.insert(Input::KeyHeld(Keycode::W), InputAction::MoveCamera(Vec3::new(0.0, 0.0, -1.0) * self.movement_speed));
        keymap.insert(Input::KeyHeld(Keycode::A), InputAction::MoveCamera(Vec3::new(-1.0, 0.0, 0.0) * self.movement_speed));
        keymap.insert(Input::KeyHeld(Keycode::S), InputAction::MoveCamera(Vec3::new(0.0, 0.0, 1.0) * self.movement_speed));
        keymap.insert(Input::KeyHeld(Keycode::D), InputAction::MoveCamera(Vec3::new(1.0, 0.0, 0.0) * self.movement_speed));
        keymap.insert(Input::KeyHeld(Keycode::ESCAPE), InputAction::Quit);
        keymap
    }

    fn handle_mouse(&self, mouse_motion: MouseMotion) -> Option<InputAction> {
        let adjusted_movement = (mouse_motion.0 as f32 * self.look_sensitivity, mouse_motion.1 as f32 * self.look_sensitivity);
        Some(InputAction::LookDelta(adjusted_movement))
    }
} 
