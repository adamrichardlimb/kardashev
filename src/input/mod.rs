use sdl2::EventPump;

pub trait Controller {
    fn handle_input();
}

pub struct Input {
    event_pump: EventPump,
}
