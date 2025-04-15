use sdl2::keyboard::Keycode;
use crate::input::FrameInput;
use std::collections::{HashMap, HashSet};
use super::InputAction;

//pub mod composite_controller;
pub mod camera_controller;

pub trait Controller {
    fn keymap(&self) -> KeyMap;
    
    fn claimed_keys(&self) -> HashSet<Keycode> {
        let mut keys = HashSet::new();
        for input in self.keymap().keys() {
            match input {
                Input::KeyPressed(keycode) => keys.insert(*keycode),
                Input::KeyHeld(keycode) => keys.insert(*keycode),
                Input::KeyReleased(keycode) => keys.insert(*keycode)
            };
        }
        keys
    }

    fn handle_input(&self, input: FrameInput) -> Vec<InputAction> {
        let mut actions = self.handle_keys(input.keys_input);
        let mouse_action = input.mouse_input.and_then(|motion| self.handle_mouse(motion));
        if let Some(mouse_action) = mouse_action {
            actions.push(mouse_action);
        }
        actions
    }

    fn handle_keys(&self, keys: HashSet<Input>) -> Vec<InputAction> {
        let mut actions = Vec::new();
        for key in keys {
            if let Some(action) = self.keymap().get(&key).cloned() {
                actions.push(action);
            }
        }
        actions
    }

    fn handle_mouse(&self, mouse_motion: MouseMotion) -> Option<InputAction> {
        None
    }
}

pub type MouseMotion = (i32, i32);

#[derive(Eq, Hash, PartialEq)]
pub enum Input {
    KeyPressed(Keycode),
    KeyHeld(Keycode),
    KeyReleased(Keycode),
}

pub type KeyMap = HashMap<Input, InputAction>;
