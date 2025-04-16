use sdl2::keyboard::Keycode;
use std::collections::HashMap;

use crate::input::InputAction;
use crate::input::{Controller, controllers::KeyMap};

use super::Input;

pub struct DebugOverlayController {}

impl Controller for DebugOverlayController {

    fn keymap(&self) -> KeyMap {
        let mut map = HashMap::new();
        map.insert(Input::KeyPressed(Keycode::F1), InputAction::ToggleDebugModule(1));
        map.insert(Input::KeyHeld(Keycode::S), InputAction::ToggleDebugModule(1));
        map
    }
}
