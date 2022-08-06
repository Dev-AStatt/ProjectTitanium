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

        let tile_sheet = TileSheet::new(ctx, "/Pokemon_Tile_sheet.png");
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
        //self.tile_sheet.draw_tile(canvas, 29, glam::Vec2::new(0.0,0.0), 3.0);
        //let next_dest = 3.0 * 16.0;
        //self.tile_sheet.draw_tile(canvas, 29, glam::Vec2::new(next_dest,0.0), 3.0);

       let tile_spacing = self.scale as i32 * 15; 
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
























