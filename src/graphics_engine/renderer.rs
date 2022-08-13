//This struct will handle the rendering of the game
use ggez::{Context, graphics};
use super::super::{
    world::{route::Route, player},
    main_state::game_state
};
use super::sheet::TileSheet;
use super::player_sheet::PlayerSheet;
use super::super::utilities::Direction;
use super::frame::Frame;

pub struct Renderer {
    tile_sheet: TileSheet,
    player_sheet: PlayerSheet,
    frame: Frame,
    screen_move: bool,
    player_dest: glam::Vec2,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Renderer {
        
        let tile_sheet = TileSheet::new(
            ctx, 
            "/titanium_tile_sheet_v01.png",
            glam::IVec2::new(16,16), //Tile Pixel Size
            glam::IVec2::new(50,50)
        );
        let frame = Frame::new(tile_sheet.tile_size().x as f32);
        let player_dest =player_dest(frame.scale_f32(), tile_sheet.tile_size().x as f32); 
        Renderer {
            tile_sheet,
            player_sheet: PlayerSheet::new(ctx),
            player_dest, 
            frame,
            screen_move: false,
            
            
        }
    }

    pub fn update(
        self: &mut Self,
        _state: &game_state::GameState,
        _player: &player::Player,
        _time_delta: f32,
    ) {
        self.frame.update();
        if self.frame.just_flipped() {
            self.player_sheet.next_midframe();
        }
    }

    pub fn draw(
        self: &mut Self,
        canvas: &mut graphics::Canvas,
        state: &game_state::GameState,
        route: &Route,
        player: &player::Player,
    ) {
        
        match state.state_type() {
            game_state::StateType::Overworld => {
                self.draw_route(canvas, route);
                self.draw_player(canvas, player);
            }
            _ => {}
        }
    }
    

    fn draw_player(
        &self,
        canvas: &mut graphics::Canvas,
        player: &player::Player,

    ) {
        self.player_sheet.draw(
            canvas, 
            player, 
            self.player_dest,
            self.frame.scale_f32(), 
            self.frame.state()
        );
    }

    


    fn draw_route(
        &self,
        canvas: &mut graphics::Canvas,
        route: &Route,
    ) {
        //get the pixel spacing between tiles
       let tile_spacing = self.frame.scale_i32() * 16; 
        for i in 0..route.tiles().len() {
            let init_dest = glam::IVec2::new(
                (i as i32 % route.size().x) * tile_spacing,
                (i as i32 / route.size().x) * tile_spacing 
            ); 
            let final_dest = init_dest.as_vec2() + self.frame.offset();
            self.tile_sheet.draw_tile(
                canvas,
                route.tile_at(i),
                final_dest,
                self.frame.scale_f32()
            );
        } 
    }

    pub fn move_screen(self: &mut Self, d: Direction) {self.frame.move_frame(d);}
    pub fn adj_scale(self: &mut Self, inc: i32) {self.frame.inc_scale(inc);}
}


fn player_dest(scale: f32, size: f32) -> glam::Vec2 {
        //screen size is 21, 20 tiles or 1008,960 pixels
        let center_tile = glam::Vec2::new(10.0, 9.0);
        let center_pixels = center_tile * scale * size;
        return center_pixels; 
    }





















