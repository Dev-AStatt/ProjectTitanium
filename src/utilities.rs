use ggez::{Context, graphics};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//Function will take a path to the sprite that wants to load and return either
//that sprite or a red cube of 5x5 if there was an error
pub fn load_image(ctx: &Context, path: &str) -> graphics::Image {
    let img = graphics::Image::from_path(ctx, path, true);

    match graphics::Image::from_path(ctx, path, true) {
        Ok(it) => {
            return it;
        },
        Err(err) => {
            println!("Pub fn sprite_get: Error on loading path: {}", path);
            println!("Error: {}", err);
            return graphics::Image::from_solid(ctx, 5, graphics::Color::RED);
        }
    };
}
