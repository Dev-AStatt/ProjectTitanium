//This struct will handle the rendering of the game
use ggez::{Context, graphics};
use crate::{world::world::Route, main_state::game_state::GameState};

use super::sheet::TileSheet;

pub struct Renderer {
    tile_sheet: TileSheet,
    screen_offset: glam::Vec2,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Renderer {
        
        let tile_sheet = TileSheet::new(
            ctx, 
            "/titanium_tile_sheet_v01.png",
            glam::IVec2::new(16,16), //Tile Pixel Size
            glam::IVec2::new(80,80)
        );
        Renderer {
            tile_sheet,
            screen_offset: glam::Vec2::new(0.0,0.0),
        }
    }
    pub fn draw_route(
        &self,
        canvas: &mut graphics::Canvas,
        route: &Route,
        state: &GameState,
    ) {
        //get the pixel spacing between tiles
       let tile_spacing = state.scale() * 16; 
        for i in 0..route.tiles().len() {
            let init_dest = glam::IVec2::new(
                (i as i32 % route.size().x) * tile_spacing,
                (i as i32 / route.size().x) * tile_spacing 
            ); 
            let final_dest = init_dest.as_vec2() + self.screen_offset;
            self.tile_sheet.draw_tile(
                canvas,
                route.tile_at(i),
                final_dest,
                state.scale()
            );
        } 
    }

    pub fn move_screen_up(&mut self, scale: i32) {
        let offset = 0.5 * self.tile_sheet.tile_size().y as f32 * scale as f32 ;
            self.adjust_screen_offset((0.0, offset));
    } 
    pub fn move_screen_down(&mut self, scale: i32) {
        let offset = 0.5 * self.tile_sheet.tile_size().y as f32 * scale as f32 ;
            self.adjust_screen_offset((0.0, -1.0 * offset));
    } 
    pub fn move_screen_left(&mut self, scale: i32) {
        let offset = 0.5 * self.tile_sheet.tile_size().x as f32 * scale as f32 ;
        self.adjust_screen_offset((offset,0.0));
    } 
    pub fn move_screen_right(&mut self, scale: i32) {
        let offset = 0.5 * self.tile_sheet.tile_size().x as f32 * scale as f32 ;
        self.adjust_screen_offset((-1.0 * offset,0.0));
    } 

    fn adjust_screen_offset(&mut self, offset: (f32, f32)) {
        //Method will check if the passed in offset will push too far off screen 
        //if gone too far down
        if self.screen_offset.x + offset.0 >= 0.0 {
            self.screen_offset.x += offset.0;
        } else {
            self.screen_offset.x = 0.0
        }
        //if gone too far right
        if self.screen_offset.y + offset.1 >= 0.0 {
            self.screen_offset.y += offset.1;
        } else {
            self.screen_offset.y = 0.0
        }
    }

}
























