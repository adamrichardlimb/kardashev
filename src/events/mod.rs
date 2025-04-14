use crate::world::chunk::ChunkBlockData;
use crate::world::ChunkPos;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use tracing::debug;
use strum_macros::Display;

#[derive(Display)]
pub enum Event {
    ChunkLoaded(ChunkPos, ChunkBlockData),
    ChunkUnloaded(ChunkPos)
}

#[derive(Eq, Hash, PartialEq)]
pub enum EventType {
    ChunkLoaded,
    ChunkUnloaded
}

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Event::ChunkLoaded(..) => EventType::ChunkLoaded,
            Event::ChunkUnloaded(..) => EventType::ChunkUnloaded,
        }
    }
}

pub trait EventHandler {
    fn on_event(&mut self, event: &Event);
    fn event_types(&self) -> Vec<EventType>;
}

pub struct EventQueue {
    events: Vec<Event>,
    listeners: HashMap<EventType, Vec<Rc<RefCell<dyn EventHandler>>>>
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            listeners: HashMap::new()
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn register_handler(&mut self, handler: Rc<RefCell<dyn EventHandler>>) {
        let types = handler.borrow().event_types();
        for event_type in types {
            self.listeners.entry(event_type).or_default().push(handler.clone());
        }
    }

    pub fn deregister_handler(&mut self, handler: Rc<RefCell<dyn EventHandler>>) {
        let types = handler.borrow().event_types();
        for event_type in types {
            if let Some(handlers) = self.listeners.get_mut(&event_type) {
                handlers.retain(|h| !Rc::ptr_eq(h, &handler));
            }
        }
    }


    pub fn dispatch_events(&mut self) {
        debug!("Dispatching {} events...", self.events.len());
        for event in self.events.drain(..) {
            if let Some(listeners) = self.listeners.get(&event.event_type()) {
            debug!("Dispatching {} event to {} listeners...", event, listeners.len());
                for listener in listeners {
                    listener.borrow_mut().on_event(&event);
                }
            }
        }
    }
}
