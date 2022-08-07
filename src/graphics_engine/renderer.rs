//This struct will handle the rendering of the game
use ggez::{Context, graphics};
use crate::world::world::Route;

use super::sheet::TileSheet;

pub struct Renderer {
    tile_sheet: TileSheet,
    scale: f32,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Renderer {
        
        let tile_sheet = TileSheet::new(
            ctx, 
            "/Pokemon_Tile_sheet.png",
            glam::IVec2::new(16,16), //Tile Pixel Size
            glam::IVec2::new(9,9)
        );
        let scale = 5.0;
        Renderer {
            tile_sheet,
            scale,
        }
    }
    pub fn draw_route(
        &self,
        canvas: &mut graphics::Canvas,
        route: &Route,
    ) {
        //get the pixel spacing between tiles
       let tile_spacing = self.scale as i32 * 16; 
        for i in 0..route.tiles().len() {
            let screen_dest = glam::IVec2::new(
                (i as i32 % route.size().x) * tile_spacing,
                (i as i32 / route.size().x) * tile_spacing 
            ); 
            self.tile_sheet.draw_tile(
                canvas,
                route.tile_at(i),
                screen_dest.as_vec2(), 
                self.scale
            );
        } 
    }
}
























