pub mod controllers;

use crate::input::controllers::{Input, Controller};
use controllers::MouseMotion;
use tracing::debug;
use std::collections::HashSet;
use glam::Vec3;
use sdl2::{event::Event, keyboard::Keycode, EventPump};

//TODO - Create an input buffer for the InputDispatcher
pub struct InputDispatcher<'a> {
    event_pump: EventPump,
    active_controller: Option<Box<dyn Controller + 'a>>,
    keys_held: HashSet<Keycode>,
    mouse_motion: Option<(i32, i32)>
}

pub struct FrameInput {
    pub keys_input: HashSet<Input>,
    pub mouse_input: Option<MouseMotion>
}

//TODO - this is temp code for emitting actions to stop CameraController possessing a mutable
//borrow indefinitely
#[derive(Clone)]
pub enum InputAction {
    MoveCamera(Vec3),
    LookDelta((f32, f32)),
    ToggleDebugModule(i32),
    Quit
}

impl<'a> InputDispatcher<'a> {
    pub fn new(event_pump: EventPump) -> InputDispatcher<'a> {
        let input_handler = InputDispatcher {
            event_pump,
            active_controller: None,
            keys_held: HashSet::new(),
            mouse_motion: None
        };
        

        return input_handler;
    }

    pub fn set_controller<C: Controller + 'a>(&mut self, controller: C) {
        debug!("Active controller changing...");
        self.active_controller = Some(Box::new(controller)); 
    }

    pub fn poll_events(&mut self) -> Result<FrameInput, String> {
        debug!("Polling for input events...");
        let mut keys_input = HashSet::new();
        self.mouse_motion = None;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(k), repeat: false, .. } => {
                    if !self.keys_held.contains(&k) {
                        keys_input.insert(Input::KeyPressed(k));
                    }
                    keys_input.insert(Input::KeyHeld(k));
                    self.keys_held.insert(k);
                }
                Event::KeyUp { keycode: Some(k), .. } => {
                    self.keys_held.remove(&k);
                    keys_input.insert(Input::KeyReleased(k));
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    self.mouse_motion = Some((xrel, yrel));
                }
                _ => {}
            }
        }

        Ok(FrameInput {
            keys_input,
            mouse_input: self.mouse_motion,
        })
    }

    pub fn update(&mut self) -> Result<Vec<InputAction>, String> {
        let input = self.poll_events()?;
        return Ok(self.active_controller
            .as_mut()
            .expect("No active controller!")
            .handle_input(input)
        );
    }
}
