
use super::json_importer::{self, TileClass};

#[derive(Clone, Copy)]
pub struct Tile {
    global_id: i32,
    class: TileClass,
    walkable: bool,
    spawn: bool,
}

#[derive(Clone)]
pub struct Route {
    size: glam::IVec2, //Starts at 0
    tiles: Vec<i32>, 
}


impl Route {
    pub fn new() -> Route {

        let import_route = json_importer::read_json_from_file(
            "../../resources/routes/titanium_town1_V02.json").unwrap();






        let tiles = import_route.data_layer(0);

        let route_size = glam::IVec2::new(
            import_route.layer(0).width() as i32,
            import_route.layer(0).height() as i32
        );

        Route{
            size: route_size,
            tiles,
        }
    }




    pub fn size(&self) -> glam::IVec2 {return self.size}
    pub fn tiles(&self) -> &Vec<i32> {return &self.tiles} 
    pub fn tile_at(&self, i: usize) -> i32 {return self.tiles[i]}

}













