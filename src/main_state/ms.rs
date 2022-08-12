//Main functions for the GGEZ engine all needs to be in one file


use ggez::{
    event::{self, MouseButton},
    graphics::{self, Drawable, Sampler},
    Context,
    GameResult,
    input::keyboard::{KeyCode, KeyInput},
};


use super::game_state::{GameState, StateType};
use super::super::{
    graphics_engine::renderer,
    world::{
        world::World,
        player::Player
    },
};



pub struct MainState {
    pub renderer: renderer::Renderer,
    pub player: Player, 
    pub world: World,
    pub state: GameState,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {


        let ms = MainState {
            renderer: renderer::Renderer::new(ctx),
            player: Player::new(),
            world: World::new(),
            state: GameState::new()
            
        };
        Ok(ms) 
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        self.renderer.update(&self.state);
        Ok(()) 
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        //Clear the screen
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear(graphics::Color::BLACK)
        );
        //Give the screen size to gamestate
        self.state.set_screen_size(glam::Vec2::new(
            canvas.screen_coordinates().unwrap().w, 
            canvas.screen_coordinates().unwrap().h
        ));
        //this sets the sampler rate to be nearest completion, for 2d sprites
        canvas.set_sampler(Sampler::nearest_clamp());

        self.renderer.draw(
            &mut canvas, 
            &self.state, 
            &self.world.current_route(), 
            &self.player
        );
        //draw any additional debug information we need 
        self.draw_debug_info(&mut canvas, ctx);


        canvas.finish(ctx)?;
        Ok(())

    }

    //The ggez engine will call events automatically for key and mouse events
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        //ensure that we are in the overworld before adjusting scale
        match self.state.state_type() {
            StateType::Overworld => {
                self.renderer.adj_scale(y as i32);
            }
            _ => (),
        }
        Ok(())
    }

    //The ggez engine will call events automatically for key and mouse events
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        //we match the state type to isolate what keys do        
        match self.state.state_type() {
            StateType::Overworld => {self.io_overworld(input)}
            _ => (),
        }
        Ok(())
    }


}






