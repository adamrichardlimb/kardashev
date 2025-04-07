use crate::world::ChunkPos;

pub enum Event {
    ChunkLoaded(ChunkPos),
    ChunkUnloaded(ChunkPos)
}

pub struct EventQueue {
    events: Vec<Event>
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new()
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn get_queue(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn clear_queue(&mut self) {
        self.events = Vec::new();
    }
}
