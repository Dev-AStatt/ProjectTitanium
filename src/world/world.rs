//struct to contain the world information

use csv;
use std::error::Error;

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
        let tiles = get_route1(); 
        Route{
            size: glam::IVec2::new(24,31),
            tiles,
        }
    }
    pub fn size(&self) -> glam::IVec2 {return self.size}
    pub fn tiles(&self) -> &Vec<i32> {return &self.tiles} 
    pub fn tile_at(&self, i: usize) -> i32 {return self.tiles[i]}
}





fn get_route_test() -> Vec<i32> {
    //size is 9,9
    let tiles = vec![
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
        ];
    return tiles;
}



fn get_route1() -> Vec<i32> {
    // 24 by 31
    let route1 = vec![
        8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,
        88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,
        248,249,168,169,168,169,168,169,168,169,168,169,168,169,168,169,168,169,168,169,168,169,248,249,
        88,89,0,0,1,0,1,0,1,0,0,1,0,0,1,0,1,0,1,0,0,1,88,89,
        248,249,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,1,1,248,249,
        88,89,1,1,0,1,1,0,0,1,0,1,1,1,0,1,1,0,0,1,0,1,88,89,
        248,249,1,1,0,1,0,0,1,0,0,0,1,1,0,1,0,0,1,0,0,0,248,249,
        88,89,0,0,0,1,1,0,1,1,1,0,0,0,0,1,1,0,1,1,1,0,88,89,
        248,249,0,0,0,0,0,1,1,0,1,0,0,0,0,0,0,1,1,0,1,0,248,249,
        88,89,0,0,1,1,0,1,1,1,0,1,0,0,1,1,0,1,1,1,0,1,88,89,
        248,249,0,0,0,0,0,1,0,1,0,1,0,0,0,0,0,1,0,1,0,1,248,249,
        88,89,0,1,1,1,0,1,0,0,1,0,0,1,1,1,0,1,0,0,1,0,88,89,
        248,249,0,1,1,1,0,0,0,0,1,1,0,1,1,1,0,0,0,0,1,1,248,249,
        88,89,1,1,1,1,0,0,1,0,0,0,1,1,1,1,0,0,1,0,0,0,88,89,
        248,249,1,0,0,1,1,0,0,0,1,0,1,0,0,1,1,0,0,0,1,0,248,249,
        88,89,0,1,0,1,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,88,89,
        248,249,1,0,0,0,1,1,0,1,0,0,1,0,0,0,1,1,0,1,0,0,248,249,
        88,89,0,1,0,0,1,1,0,0,1,1,0,1,0,0,1,1,0,0,1,1,88,89,
        248,249,1,1,0,1,0,1,0,0,1,0,1,1,0,1,0,1,0,0,1,0,248,249,
        88,89,1,1,0,0,1,1,0,0,0,0,1,1,0,0,1,1,0,0,0,0,88,89,
        248,249,0,1,1,1,0,0,0,1,1,0,0,1,1,1,0,0,0,1,1,0,248,249,
        88,89,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,0,1,1,88,89,
        248,249,1,0,1,1,0,1,0,1,1,0,1,0,1,1,0,1,0,1,1,0,248,249,
        88,89,1,0,0,0,1,0,1,1,0,1,1,0,0,0,1,0,1,1,0,1,88,89,
        248,249,0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,1,0,0,1,248,249,
        88,89,1,0,1,0,0,1,1,1,0,1,1,0,1,0,0,1,1,1,0,1,88,89,
        248,249,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,0,1,248,249,
        88,89,1,0,0,0,1,0,0,1,1,0,1,0,0,0,1,0,0,1,1,0,88,89,
        248,249,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,248,249,
        88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,88,89,
        167,168,168,169,168,169,168,169,168,169,168,169,168,169,168,169,168,169,168,169,168,169,167,168
    ];
    return route1;
}






