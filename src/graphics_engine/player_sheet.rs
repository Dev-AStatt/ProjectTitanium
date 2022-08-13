
use ggez::{graphics, Context};
use super::super::utilities;
use super::super::world::player;
use super::frame::FrameState;


#[derive(PartialEq)] 
pub enum SpriteMidFrame {
    Left,
    Right,
}
impl SpriteMidFrame {
    pub fn next(&self) -> SpriteMidFrame {
        match self {
            SpriteMidFrame::Left => {return SpriteMidFrame::Right;}
            SpriteMidFrame::Right => {return SpriteMidFrame::Left;}
        }
    }
}


pub struct PlayerSheet {
    image: graphics::Image,
    image_size: glam::Vec2,    
    midframe: SpriteMidFrame,
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
            midframe: SpriteMidFrame::Left,
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
        let rect = self.calc_player_rect(player, frame);  
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
        player: &player::Player,
        frame: FrameState,
    ) -> graphics::Rect {

        //Start with the idle sprite down at 0,0 
        let mut img_xy = glam::UVec2::new(0,0);
        //match the row for what action is hammening
        match player.current_action() {
            player::PlayerAction::Idle => {}        //Do nothing
            player::PlayerAction::Walking => {
                if frame == FrameState::Mid {
                    //if we are in a midframe, show the right or left midframe
                    match self.midframe {
                        SpriteMidFrame::Right => {img_xy.y += 2}
                        SpriteMidFrame::Left => {img_xy.y += 4}
                    } 
                }
            }
            player::PlayerAction::Running => {}
        }

        //match the direction of the player
        match player.direction() {
            utilities::Direction::Up => {img_xy.x += 1}
            utilities::Direction::Down => {}
            utilities::Direction::Left => {img_xy.x += 2}
            utilities::Direction::Right => {img_xy.x += 3}
        }


        let pixel_xy = img_xy * self.sprite_size;

        let rect: graphics::Rect = graphics::Rect::new(
            pixel_xy.x as f32 / self.image_size.x,
            pixel_xy.y as f32 / self.image_size.y,
            self.sprite_pixel_rect_wh.x,
            self.sprite_pixel_rect_wh.y
        );
        return rect;
    }

    pub fn next_midframe(&mut self) {self.midframe = self.midframe.next()}
}










