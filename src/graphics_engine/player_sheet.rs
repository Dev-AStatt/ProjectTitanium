
use ggez::{graphics, Context};
use super::super::utilities;
use super::super::world::player;
use super::frame::FrameState;


#[derive(PartialEq)] 
pub enum Midframe {
    Left,
    Right,
}
impl Midframe {
    pub fn next(&self) -> Midframe {
        match self {
            Midframe::Left => {return Midframe::Right;}
            Midframe::Right => {return Midframe::Left;}
        }
    }
}


pub struct PlayerSheet {
    image: graphics::Image,
    image_size: glam::Vec2,    
    midframe: Midframe,
    sprite_rows: u32,
    sprite_cols: u32,
    sprite_size: glam::UVec2,
    sprite_pixel_rect_wh: glam::Vec2,
}

impl PlayerSheet {
    pub fn new(
        ctx: &mut Context,
    ) -> PlayerSheet {
        let image = utilities::load_image(ctx, "/boy_sprite_sheet.png");
        let image_size = glam::Vec2::new(image.width() as f32, image.height() as f32);
        let sprite_size = glam::UVec2::new(16,16);
        //sprite hight is going to be times 2 because charicter sprites are 32 tiles high
        let sprite_pixel_rect_wh = glam::Vec2::new(
            sprite_size.x as f32 / image_size.x,
            (2 * sprite_size.y) as f32 / image_size.y,
        );
        PlayerSheet {
            image,
            image_size,
            midframe: Midframe::Left,
            sprite_rows: (image_size.y as u32 / sprite_size.y),
            sprite_cols: (image_size.x as u32 / sprite_size.x),
            sprite_size,
            sprite_pixel_rect_wh,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        player: &player::Player,
        dest: glam::Vec2,
        scale: f32,
        frame: FrameState, 
    ) {
        let rect = self.calc_player_rect(player.current_action(), frame);  
        canvas.draw(
            &self.image,
            graphics::DrawParam::new()
                .src(rect)
                .dest(dest)
                .scale([scale * rect.w, scale * rect.h])
        );
    }

    //function will select the correct sprite to display 
    fn calc_player_rect(
        &self, 
        action: player::PlayerAction,
        frame: FrameState,
    ) -> graphics::Rect {

        //Start with the idle sprite down at 0,0 
        let img_xy = glam::UVec2::new(0,0);
        //match the row for what action is hammening
        match action {
            player::PlayerAction::Idle => {}        //Do nothing
            player::PlayerAction::Walking => {
            }
            player::PlayerAction::Running => {}
        }
        let pixel_xy = img_xy * self.sprite_size;

        let rect: graphics::Rect = graphics::Rect::new(
            pixel_xy.x as f32 / self.image_size.x,
            pixel_xy.y as f32 / self.image_size.x,
            self.sprite_pixel_rect_wh.x,
            self.sprite_pixel_rect_wh.y
        );
        return rect;
    }

}










