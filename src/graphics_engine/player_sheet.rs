
use ggez::{graphics, Context};
use super::super::utilities;
use super::super::world::player;


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
}

impl PlayerSheet {
    pub fn new(
        ctx: &mut Context,
    ) -> PlayerSheet {
        let image = utilities::load_image(ctx, "/boy_sprite_sheet.png");
        let image_size = glam::Vec2::new(image.width() as f32, image.height() as f32);
        PlayerSheet {
            image,
            image_size,
            midframe: Midframe::Left,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        player: &player::Player,
        dest: glam::Vec2,
        scale: i32,
    ) {
        let rect = self.calc_player_rect(player.current_action());  
        canvas.draw(
            &self.image,
            graphics::DrawParam::new()
                .src(rect)
                .dest(dest)
                .scale([scale as f32 * rect.w, scale as f32 * rect.h])
        );
    }

    fn calc_player_rect(
        &self, 
        action: player::PlayerAction
    ) -> graphics::Rect {




        let rect: graphics::Rect;
        return rect;

    }



}










