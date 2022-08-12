//This struct will handle the rendering of the game
use ggez::{Context, graphics};
use super::super::{
    world::{route::Route, player},
    main_state::game_state
};
use super::sheet::TileSheet;
use super::player_sheet::PlayerSheet;
use super::super::utilities::Direction;
use super::frame::{Frame, FrameState};

pub struct Renderer {
    tile_sheet: TileSheet,
    player_sheet: PlayerSheet,
    frame: Frame,
    screen_move: bool,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Renderer {
        
        let tile_sheet = TileSheet::new(
            ctx, 
            "/titanium_tile_sheet_v01.png",
            glam::IVec2::new(16,16), //Tile Pixel Size
            glam::IVec2::new(50,50)
        );
        let frame = Frame::new();
        Renderer {
            tile_sheet,
            player_sheet: PlayerSheet::new(ctx),
            frame,
            screen_move: false,
        }
    }

    pub fn update(
        self: &mut Self,
        state: &game_state::GameState,
    ) {
        self.frame.update(self.tile_sheet.tile_size().x as f32);
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
                self.draw_route(canvas, state, route);
                self.draw_player(canvas, state, player);
            }
            _ => {}
        }
    }
    

    fn draw_player(
        &self,
        canvas: &mut graphics::Canvas,
        state: &game_state::GameState,
        player: &player::Player,

    ) {
        let dest = self.player_dest(state);
        self.player_sheet.draw(
            canvas, 
            player, 
            dest,
            self.frame.scale_f32(), 
            self.frame.state()
        );
    }

    fn player_dest(
        &self, 
        state: &game_state::GameState,
    ) -> glam::Vec2 {
        let screen_center_tile = state.screen_size() / 2.0;
        return screen_center_tile; 
    }


    fn draw_route(
        &self,
        canvas: &mut graphics::Canvas,
        state: &game_state::GameState,
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

    pub fn move_screen(self: &mut Self, d: Direction) {
        self.frame.move_frame(d, self.tile_sheet.tile_size().x as f32);
    }
    pub fn adj_scale(self: &mut Self, inc: i32) {self.frame.inc_scale(inc);}
}
























