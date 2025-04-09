use crate::world::chunk::ChunkBlockData;
use crate::world::ChunkPos;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

    pub fn register_listener(&mut self, event_types: Vec<EventType>, listener: Rc<RefCell<dyn EventHandler>>) {
        for event_type in event_types {
            self.listeners.entry(event_type).or_default().push(listener.clone());
        }
    }

    pub fn dispatch_events(&mut self) {
        println!("Dispatching {} events...", self.events.len());
        for event in self.events.drain(..) {
            if let Some(listeners) = self.listeners.get(&event.event_type()) {
                for listener in listeners {
                    listener.borrow_mut().on_event(&event);
                }
            }
        }
    }
}
