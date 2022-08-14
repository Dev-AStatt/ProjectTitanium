
use super::json_importer::{self, TileClass, ImportedRouteJson};

#[derive(Debug,Clone, Copy)]
pub struct Tile {
    global_id: i32,
    class: TileClass,
    walkable: bool,
    spawner: bool,
}
impl Tile {
    pub fn new(
        global_id: i32,
        class: TileClass,
        walkable: bool,
        spawner: bool,
    ) -> Tile {
       Tile { global_id, class, walkable, spawner } 
    }
    pub fn global_id(&self) -> i32 {self.global_id}
    pub fn class(&self) -> TileClass {self.class}
    pub fn walkable(&self) -> bool {self.walkable}
    pub fn spawner(&self) -> bool {self.spawner}
}

#[derive(Clone)]
pub struct Route {
    size: glam::IVec2, //Starts at 0
    tiles: Vec<Tile>, 
}


impl Route {
    pub fn new() -> Route {

        let import_route = json_importer::read_json_from_file(
            "../../resources/routes/titanium_town1_V02.json").unwrap();
        
        let tiles = build_tiles_from_import(&import_route);

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
    pub fn tiles(&self) -> &Vec<Tile> {return &self.tiles} 
    pub fn tile_id_at(&self, i: usize) -> i32 {return self.tiles[i].global_id()}

}

fn build_tiles_from_import(imp_route: &ImportedRouteJson) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];
    let layer = 0;
    let num_tilesets = imp_route.num_tilesets();

    println!("{:?}",imp_route.tilesets()[0].tiles);
    for i in 0..imp_route.layer(layer).data_len() {
       
        let global_id = imp_route.layer(layer).data_by_ref()[i];
        println!("GlobalID: {}", global_id);
        let tileset_id: usize = global_id as usize - 1;
        let class = imp_route.tilesets()[0].get_tile_class(tileset_id); 
        let walkable = imp_route.tilesets()[0].is_tile_walkable(tileset_id); 
        let spawner = imp_route.tilesets()[0].is_tile_spawner(tileset_id); 

        let new_tile = Tile::new(
            global_id,
            class,
            walkable,
            spawner,
        );
        println!("Properly imported {:?}", new_tile);
        tiles.push(new_tile);
    }

    return tiles;
}

//    global_id: i32,
//    class: TileClass,
//    walkable: bool,
//    spawn: bool,










