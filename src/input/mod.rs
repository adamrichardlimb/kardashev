use sdl2::{event::Event, keyboard::Keycode, EventPump, EventSubsystem};
use std::collections::HashMap;

pub fn init(event_pump: EventPump, event_subsystem: EventSubsystem) -> InputHandler {
    let keymap = HashMap::new();
    let mut input_handler = InputHandler {
        event_pump,
        keymap
    };
    
    let event_sender = event_subsystem.event_sender();

    input_handler.bind(Keycode::SPACE, move || {let _ = event_sender.push_event(Event::Quit { timestamp: 12345 });} );
    input_handler.bind(Keycode::TAB, || println!("Hello, world!"));

    return input_handler;
}

pub struct InputHandler {
    event_pump: EventPump,
    keymap: Keymap
}

pub type Keymap = HashMap<Keycode, Box<dyn FnMut()>>;

impl InputHandler {
    pub fn bind<F>(&mut self, key: Keycode, action: F) where F: FnMut() + 'static, {
        self.keymap.insert(key, Box::new(action));
    }

    fn handle_key(&mut self, key: Keycode) {
        if let Some(action) = self.keymap.get_mut(&key) {
            action();
        }
    }

    pub fn poll_events(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Err("User wishes to close program".to_string()),
                Event::KeyDown { keycode: Some(k), .. } => {
                        self.handle_key(k);
                        return Ok(());
                },
                _ => return Ok(())
            }
        }

        return Ok(());
    }
}
