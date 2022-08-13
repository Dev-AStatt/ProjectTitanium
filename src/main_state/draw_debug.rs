//This file holds the draw calls for any debug information we need druing development

use ggez::{
    graphics,
    Context,
};

use super::ms;

impl ms::MainState {
    pub fn draw_debug_info(
        &self,
        _canvas: &mut graphics::Canvas,
        ctx: &mut Context,
    ) {
        
        //Draw an FPS Counter in the top bar 
        ctx.gfx.set_window_title(&format!(
            "Pokemon Titanium - {:.0} FPS", ctx.time.fps()
        ));
    }
}





