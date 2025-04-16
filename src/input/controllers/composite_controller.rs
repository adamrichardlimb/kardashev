use crate::input::{Controller, controllers::KeyMap};
use std::collections::{HashMap, HashSet};
use tracing::debug;

pub struct CompositeController {
    layers: Vec<Box<dyn Controller>>
}

impl CompositeController {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
        }
    }

    pub fn push_controller(&mut self, controller: Box<dyn Controller>) {
        self.layers.push(controller);
    }

    pub fn pop_layer(&mut self) {
        self.layers.pop();
    }
}

impl Controller for CompositeController {
    fn keymap(&self) -> KeyMap {
       let mut map = HashMap::new();
       let mut claimed_keys = HashSet::new();

       for layer in self.layers.iter().rev() {
           for (input, action) in layer.keymap() {
               if !claimed_keys.contains(&input) {
                   map.insert(input.clone(), action);
                   claimed_keys.insert(input);
               } else {
                   debug!("Input collision detected!");
               }
           }
       }

       map
    }
}
