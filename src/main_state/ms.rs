//Main functions for the GGEZ engine all needs to be in one file


use ggez::{
    event::{self, MouseButton},
    graphics,
    Context,
    GameResult,
    input::keyboard::{KeyCode, KeyInput},
};

pub struct MainState {

}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(
            MainState {

            }
        )
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

    

    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(()) 
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
       
        //Clear the screen
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1,0.1,0.1,1.0].into())
        );
        Ok(())

    }

}






