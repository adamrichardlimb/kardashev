use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump};

pub struct Sdl2Utils {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump
}

impl Sdl2Utils {

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 200, 0));
        self.canvas.clear();

        // Draw a red rectangle
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(Rect::new(100, 100, 600, 400));

        // Show it on the screen
        self.canvas.present();
    }

    pub fn poll_events(&mut self) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Err("User wishes to close program.".to_string()),
                _ => return Ok(())
            }
        }

        return Ok(());
    }

}
