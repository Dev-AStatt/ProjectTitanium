//Main functions for the GGEZ engine all needs to be in one file


use ggez::{
    event::{self, MouseButton},
    graphics,
    Context,
    GameResult,
    input::keyboard::{KeyCode, KeyInput},
};

use super::super::world::player::Player;
use super::super::utilities;
use super::super::renderer;
 
pub struct MainState {
    pub player: Player, 
    pub tile_sheet: graphics::Image,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {

        let img = utilities::load_image(ctx, "/tile_sheet_V01.png");

        let ms = MainState {
            player: Player::new(),
            tile_sheet: img, 
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

        self.draw_debug_info(&mut canvas, ctx);
        let tiles_in_page = glam::IVec2::new(18, 26);
        let tile_scale_down = glam::Vec2::new(1.0/ tiles_in_page.x as f32, 1.0 / tiles_in_page.y as f32 ); 
        let test_texture = graphics::Rect::new(0.5,0.5, 1.0, 1.0);

        let scale = 1.0;
        canvas.draw(
            &self.tile_sheet,
            graphics::DrawParam::new()
                .src(test_texture)
                .dest([0.0,0.0])
                .scale([scale * test_texture.w , scale * test_texture.h ])
        );
        canvas.finish(ctx)?;
        Ok(())

    }

}






