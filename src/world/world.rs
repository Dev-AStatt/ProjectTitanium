//struct to contain the world information

use super::route::Route;

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



