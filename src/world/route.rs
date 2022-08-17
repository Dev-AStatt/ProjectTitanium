

use super::super::utilities::Direction;
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

        //import Json file for route 
        let import_route = json_importer::read_json_from_file(
            "../resources/routes/titanium_town1_V02.json").unwrap();
        //Build the vector of tiles from imported Json
        let tiles = build_tiles_from_import(&import_route);
        let route_size = glam::IVec2::new(
            import_route.layer(0).width() as i32,
            import_route.layer(0).height() as i32
        );
        println!("Route Size {}", route_size);

        Route{
            size: route_size,
            tiles,
        }
    }
    pub fn size(&self) -> glam::IVec2 {return self.size}
    pub fn tiles(&self) -> &Vec<Tile> {return &self.tiles} 
    pub fn tile_id_at(&self, i: usize) -> i32 {return self.tiles[i].global_id()}
    pub fn is_tile_walkable(&self, pos: glam::IVec2) -> bool {
        return self.tiles[self.index_from_ivec2(pos)].walkable();
    }
    pub fn is_tile_spawner(&self, pos: glam::IVec2) -> bool {
        return self.tiles[self.index_from_ivec2(pos)].spawner();
    }
    pub fn next_tile(&self, pos: glam::IVec2, dir: Direction) -> &Tile {
        let mut new_tile = pos;
        match dir {
            Direction::Up => {new_tile.y -= 1}
            Direction::Down => {new_tile.y += 1}
            Direction::Left => {new_tile.x -= 1}
            Direction::Right => {new_tile.x += 1}
        } 
        return &self.tiles[self.index_from_ivec2(new_tile)]

    }
    fn index_from_ivec2(&self, p: glam::IVec2) -> usize {
        return (p.x + (p.y * self.size.x)) as usize;
    }

}

//Function handeles the translation from importing the Tiled json export into the 
//usable structs that we need to use in the game. Currently this is building a 
//vector of tiles Vec<Tile> that will be our map.
fn build_tiles_from_import(imp_route: &ImportedRouteJson) -> Vec<Tile> {
    //Written - ASTATT - 2022-08-15
    //NOTE on Global_ID, global_ID is the tilesheet id + 1 because Tiled will export 0 for no tile

    //create empty vector
    let mut tiles: Vec<Tile> = vec![];
    //we will assume only one layer for now. to be changed later
    let layer = 0;
    //Tild lets you have multple tilesets for each "route" or map. We will try not
    //to do this, so we get how many tile sets we have imported and make sure its only "1"
    //else print an error message to remind us of this
    if imp_route.num_tilesets() > 1 {println!("ERROR in Route.rs - Build Tiles from Import not setup for multple tilesets")}
    let tile_set_to_import = imp_route.num_tilesets() - 1;

    //for each global id that was imported into data. This is the bulk of what tile is where in a
    //2d vector. we have to specify the layer (always 0) that we want the data from.
    for i in 0..imp_route.layer(layer).data_len() {
        //get the global ID of the tile at index i. this is what tile on the sheet is there.
        let global_id = imp_route.layer(layer).data_by_ref()[i];
        //So the tilesheet export json will only contain data for the tiles that actually have an
        //image associated with them. So we will ask the imported tileset class to get us the
        //vector id of the imported tile associated with the global id that we are looking for data
        //at. 
        //Example. All global ID's of 2 will get a return of vector id 1 but later tiles say 50
        //will have a lower id, because tiles after 15 dont exist. 
        let tileset_id = imp_route.tilesets()[tile_set_to_import]
            .get_tiles_vecid_from_global(global_id);
        //The next three methods are all the same type of call, use the tileset_id to get what data
        //is associated with that tile. 
        let class = imp_route.tilesets()[tile_set_to_import]
            .get_tile_class(tileset_id); 
        let walkable = imp_route.tilesets()[tile_set_to_import]
            .is_tile_walkable(tileset_id); 
        let spawner = imp_route.tilesets()[tile_set_to_import]
            .is_tile_spawner(tileset_id); 

        //make a new tile struct using what we have learned
        let new_tile = Tile::new(
            global_id,
            class,
            walkable,
            spawner,
        );
        //push new tile into vector that we are creating
        tiles.push(new_tile);
    }
    return tiles;
}









