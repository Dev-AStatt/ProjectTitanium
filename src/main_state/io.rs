
use ggez::input::keyboard::{KeyCode, KeyInput};
use super::ms;
use super::super::utilities::Direction;

impl ms::MainState {

    pub fn io_overworld(&mut self, input: KeyInput) {
        
        match input.keycode {
            Some(KeyCode::Up   ) => {self.update_movement(Direction::Up)}
            Some(KeyCode::Down ) => {self.update_movement(Direction::Down)}
            Some(KeyCode::Left ) => {self.update_movement(Direction::Left)}
            Some(KeyCode::Right) => {self.update_movement(Direction::Right)}
            _ => (),
        }
    }

    fn update_movement(&mut self, d: Direction) {
        self.renderer.move_screen(d);
        self.player.set_direction(d);
    }
}





