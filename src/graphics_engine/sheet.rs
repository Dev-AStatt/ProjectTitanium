
use ggez::{graphics, Context};
use super::super::utilities;

pub struct TileSheet {
    image: graphics::Image,
    img_tile_buf: i32,
    tiles_col_row: glam::IVec2,
    tile_size: glam::IVec2,
    pixel_wh: glam::Vec2,
}

impl TileSheet {
    pub fn new(
        ctx: &mut Context,
        img_path: &str,
        tile_size: glam::IVec2,
        tiles_col_row: glam::IVec2,
    ) -> TileSheet {
        
        let image = utilities::load_image(ctx, img_path);
        let pixel_wh = glam::Vec2::new(
            (tile_size.x) as f32 / (image.width() as f32 ) ,
            (tile_size.y) as f32 / (image.height() as f32)
        );


        TileSheet {
            image,
            img_tile_buf: 1,
            tiles_col_row,
            tile_size,
            pixel_wh,
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
            tile % self.tiles_col_row.x,
            tile / self.tiles_col_row.x
        );
        let pixel_xy = (
            (tile_xy.0 * self.tile_size.x) + (tile_xy.0 * 1 + self.img_tile_buf),
            (tile_xy.1 * self.tile_size.y) + (tile_xy.1 * 1 + self.img_tile_buf)
        );
         let rect = graphics::Rect::new(
            pixel_xy.0 as f32 / self.image.width() as f32,
            pixel_xy.1 as f32 / self.image.height() as f32,
            self.pixel_wh.x,
            self.pixel_wh.y,
        );
        return rect;
    }
    //Getters and Setters
    pub fn tile_size(&self) -> glam::IVec2 {return self.tile_size}

}




