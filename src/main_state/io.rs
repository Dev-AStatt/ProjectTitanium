
use ggez::input::keyboard::{KeyCode, KeyInput};
use super::ms;
use super::super::utilities::Direction;

impl ms::MainState {

    pub fn io_overworld(&mut self, input: KeyInput) {
        
        match input.keycode {
            Some(KeyCode::Up   ) => {self.renderer.move_screen(Direction::Up)}
            Some(KeyCode::Down ) => {self.renderer.move_screen(Direction::Down)}
            Some(KeyCode::Left ) => {self.renderer.move_screen(Direction::Left)}
            Some(KeyCode::Right) => {self.renderer.move_screen(Direction::Right)}
            _ => (),
        }
    }
}





