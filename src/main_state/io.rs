
use ggez::input::keyboard::{KeyCode, KeyInput};
use super::ms;

impl ms::MainState {

    pub fn io_overworld(&mut self, input: KeyInput) {
        
        match input.keycode {
            Some(KeyCode::Up   ) => {self.renderer.move_screen_up(self.state.scale())}
            Some(KeyCode::Down ) => {self.renderer.move_screen_down(self.state.scale())}
            Some(KeyCode::Left ) => {self.renderer.move_screen_left(self.state.scale())}
            Some(KeyCode::Right) => {self.renderer.move_screen_right(self.state.scale())}
            _ => (),
        }
    }
    


}





