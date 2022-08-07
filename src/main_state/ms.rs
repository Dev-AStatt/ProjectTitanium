//Main functions for the GGEZ engine all needs to be in one file


use ggez::{
    event::{self, MouseButton},
    graphics::{self, Drawable, Sampler},
    Context,
    GameResult,
    input::keyboard::{KeyCode, KeyInput},
};

use crate::world::world::World;

use super::super::world::player::Player;
use super::super::graphics_engine::renderer;
 
pub struct MainState {
    pub renderer: renderer::Renderer,
    pub player: Player, 
    pub world: World,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {


        let ms = MainState {
            renderer: renderer::Renderer::new(ctx),
            player: Player::new(),
            world: World::new(),
        };
        Ok(ms) 
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
            graphics::CanvasLoadOp::Clear(graphics::Color::BLACK)
        );

        canvas.set_sampler(Sampler::nearest_clamp());
        self.renderer.draw_route(&mut canvas, self.world.current_route());
        


        canvas.finish(ctx)?;
        Ok(())

    }

}






