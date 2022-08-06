//struct to contain the world information

use glam::IVec2;


pub struct World {
    current_route: Route,
    
}

impl World {
    pub fn new() -> World {
        World {
            current_route: Route::new(),
        }
    }
    pub fn current_route(&self) -> &Route {return &self.current_route}
}


#[derive(Clone)]
pub struct Route {
    size: glam::IVec2, //Starts at 0
    tiles: Vec<i32>, 
}

impl Route {
    pub fn new() -> Route {
        let tiles = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 5, 5, 5, 5, 5, 5, 5, 0,
            0, 5, 5, 5, 5, 5, 5, 5, 0,
            0, 5, 5, 5, 5, 5, 5, 5, 0,
            0, 5, 5, 5, 5, 5, 5, 5, 0,
            0, 5, 96, 5, 5, 5, 5, 5, 0,
            0, 5, 5, 5, 5, 5, 5, 5, 0,
            0, 5, 5, 5, 5, 5, 5, 5, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        Route{
            size: glam::IVec2::new(8,8),
            tiles,
        }
    }
    pub fn size(&self) -> glam::IVec2 {return self.size}
    pub fn tiles(&self) -> &Vec<i32> {return &self.tiles} 
}


