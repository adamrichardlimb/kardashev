pub mod controllers;

use glam::Vec3;
use sdl2::{event::Event, keyboard::Keycode, EventPump};

//TODO - Create an input buffer for the InputDispatcher
//Maybe input buffers should sit in the controllers too?
pub struct InputDispatcher<'a> {
    event_pump: EventPump,
    active_controller: Option<Box<dyn Controller + 'a>>
}

//TODO - this is temp code for emitting actions to stop CameraController possessing a mutable
//borrow indefinitely
pub enum InputAction {
    MoveCamera(Vec3),
    Quit
}

pub trait Controller {
    fn handle_key(&mut self, key: Keycode) -> Option<InputAction>;
}

impl<'a> InputDispatcher<'a> {
    pub fn new(event_pump: EventPump) -> InputDispatcher<'a> {
        let input_handler = InputDispatcher {
            event_pump,
            active_controller: None
        };
        

        return input_handler;
    }

    pub fn set_controller<C: Controller + 'a>(&mut self, controller: C) {
       self.active_controller = Some(Box::new(controller)); 
    }

    pub fn poll_events(&mut self) -> Result<Vec<InputAction>, String> {
        let mut actions = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(k), .. } => {
                        if let Some(action) = self.active_controller
                            .as_mut()
                            .expect("No active controller!")
                            .handle_key(k)
                        {
                            actions.push(action);
                        }
                },
                _ => {}
            }
        }

        return Ok(actions);
    }
}
