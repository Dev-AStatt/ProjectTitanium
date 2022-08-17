
use ggez::input::keyboard::{KeyCode, KeyInput};
use super::ms;
use super::super::utilities::Direction;

impl ms::MainState {

    pub fn io_overworld(&mut self, input: KeyInput, _repeated: bool) {
        if !self.renderer.frame_is_complete() {return}
        match input.keycode {
            Some(KeyCode::Up   ) => {self.update_movement(Direction::Up)}
            Some(KeyCode::Down ) => {self.update_movement(Direction::Down)}
            Some(KeyCode::Left ) => {self.update_movement(Direction::Left)}
            Some(KeyCode::Right) => {self.update_movement(Direction::Right)}
            _ => (),
        }
    }

    fn update_movement(&mut self, d: Direction) {
         
        let new_tile = self.world.current_route()
            .next_tile(
                self.player.position(),
                d);
        if !new_tile.walkable() {return}
            
        self.player.inc_position_in_direction(d);
        self.renderer.move_screen(d);
    }
}





