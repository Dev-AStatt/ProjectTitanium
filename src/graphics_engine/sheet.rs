
use ggez::{graphics, Context};
use super::super::utilities;

pub struct TileSheet {
    image: graphics::Image,
    img_size: glam::UVec2,
    img_tile_buf: i32,
    tile_col_row: glam::IVec2,
    tile_size: glam::IVec2,
}

impl TileSheet {
    pub fn new(
        ctx: &mut Context,
        img_path: &str
    ) -> TileSheet {
        
        let image = utilities::load_image(ctx, img_path);
         
        let img_size = glam::UVec2::new(image.width(), image.height());

        TileSheet {
            image,
            img_size,
            img_tile_buf: 1,
            tile_col_row: glam::IVec2::new(2 , 2),
            tile_size: glam::IVec2::new(16,16),
        }
    }
    pub fn draw_tile(
        &self,
        canvas: &mut graphics::Canvas,
        tile: i32, 
        dest: glam::Vec2,
        scale: f32,
    ) {
        let rect = self.calc_tile_rect(tile);
        


        canvas.draw(
            &self.image,
            graphics::DrawParam::new()
                .src(rect)
                .dest(dest)
                .scale([scale * rect.w, scale * rect.h])
        );
        
    }
    fn calc_tile_rect(&self, tile: i32) -> graphics::Rect {
        let tile_xy = (
            tile % self.tile_col_row.x,
            tile / self.tile_col_row.x
        );
        println!("Tile_XY: X: {}, Y: {}", tile_xy.0, tile_xy.1);
        let pixel_xy = (
            tile_xy.0 * self.tile_size.x,
            tile_xy.1 * self.tile_size.y
        );
        println!("Tile_Pixels: X: {}, Y: {}", pixel_xy.0, pixel_xy.1);
        let pixel_wh = (
            (self.tile_size.x ) as f32 / (self.image.width() as f32 ) ,
            (self.tile_size.y ) as f32 / (self.image.height() as f32)
        );

        println!("Pixel_WH: W: {}, H: {}", pixel_wh.0, pixel_wh.1);
         let rect = graphics::Rect::new(
            pixel_xy.0 as f32 / self.image.width() as f32,
            pixel_xy.1 as f32 / self.image.height() as f32,
            pixel_wh.0,
            pixel_wh.1,
        );
        return rect;
    }
    //Getters and Setters
    pub fn tile_size(&self) -> glam::IVec2 {return self.tile_size}

}




