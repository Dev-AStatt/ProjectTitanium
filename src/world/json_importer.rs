/* ASTATT - 2022-08-15
* File holds code for importing Json files from the Tild Map editor.
* Tild will export the map allong with the properties for the tileset
* including things like custom properties and Class that all can be used
* by our game to have tile properties be kept in a visual system instead of a 
* big vector of intagers. 
*
* There are some gat-chas in here so take a look at the notes left for methods 
* and I hope that when properties need to be added or this needs to be expanded to 
* include multiple layers it should go ok. I have done my best to do some of the 
* work to not make it super locked into one layer. 
*/

extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use std::{
    fs::File,
    io::BufReader,
    path::Path,
    error::Error,
};

#[derive(Debug,Clone, Copy)]
pub enum TileClass {
    Normal,
    Jump,
    Door,
}


#[derive(serde_derive::Deserialize, Debug)]
//Top Layer of the Imported Json file, holds everything
pub struct ImportedRouteJson {
    compressionlevel: i8,
    layers: Vec<ImportedLayers>,
    tileheight: u32,
    tilewidth: u32,
    tilesets: Vec<ImportedTilesets>
}
impl ImportedRouteJson {
    pub fn layer(&self, u: usize) -> &ImportedLayers {return &self.layers[u]}
    //get number of tilesets that were imported, used to check to make sure we only use 1
    pub fn num_tilesets(&self) -> usize {return self.tilesets.len()}
    pub fn tilesets(&self) -> &Vec<ImportedTilesets> {return &self.tilesets}
}
#[derive(serde_derive::Deserialize, Debug)]
//Struct of a single layer that is imported.
//data is the vector of GLobal_Tile_ID's from top left to bottom right.
//data values are +1 to what is the actual position of the tile on 
//the tilesheet. This is so tiled can export 0 if no tile exists
pub struct ImportedLayers {
    data: Vec<i32>,
    height: u32,
    width: u32,
}
impl ImportedLayers {
    pub fn data_by_ref(&self) -> &Vec<i32> {return &self.data}
    pub fn data_len(&self) -> usize {return self.data.len()}
    pub fn width(&self) -> u32 {return self.width}
    pub fn height(&self) -> u32 {return self.height}
}

#[derive(serde_derive::Deserialize, Debug)]
//Tiled can export the information about the tileset used to build the map. 
//THis can include custom properties, in our case if a tile is walkable or spanws pokemon.
//the firstgid property is the id of the tileset. So should be 1. 
pub struct ImportedTilesets {
    firstgid: u32,
    pub tiles: Vec<ImportedTile>,
}
impl ImportedTilesets {
    //NOTE i: Usize used in this IMPL does NOT refer to the GLobal_Tile_ID
    //that is used in the map gen. The i used here is the id of the imported tile vector 
    //that contains ONLY the tiels with data in them. so it will jump from 
    //GLobal_Tile_ID: 15 to 50 in terms of ImportedTile.ID 
    //Use get_tiles_vecid_from_global to convert

    //returns the number of tiles that were imported into the sheet
    pub fn num_tiles_in_imported_sheet(&self) -> usize {return self.tiles.len()}
    pub fn firstgid(&self) -> u32 {self.firstgid}
    //return walkable flag of vector ID. 
    pub fn is_tile_walkable(&self, i: usize) -> bool {self.tiles[i].flag_val("Walkable".to_string())}
    pub fn is_tile_spawner(&self, i: usize) -> bool {self.tiles[i].flag_val("Spawn".to_string())}
    //Function will return enum of tileclass from the imported String taht 
    //tild exports the Class property in. Match with known used tileClass and print error if missed
    pub fn get_tile_class(&self, i: usize) -> TileClass {
        
        if i > self.tiles.len() {println!("ERROR HERE I: {} > Tile Length {}", i, self.tiles.len())}

        let str = self.tiles[i].class.clone();
        if str == "Normal" {return TileClass::Normal;}
        if str == "Jump" {return TileClass::Jump;}
        if str == "Door" {return TileClass::Door;}
        
        println!("Jaon Import Tile Error - Found unknown class: {} at ID: {}", str, i);
        return TileClass::Normal;
    }
    //as explained above in the Note, you must convert the GLobal_Tile_ID to 
    //the vector[i] to reference properties of a tile because of the jumps in imported tile_ids
    pub fn get_tiles_vecid_from_global(&self, glob: i32) -> usize {
        //search in all tiles that were imported into vector tiles
        for i in 0..self.tiles.len() {
            //if tile id matches imported global id - 1 
            //(to correct for GLobal_Tile_ID + 1 json export)
            if self.tiles[i].id() == (glob - 1) as usize {return i}
        }
        println!("Json Import Error - Get_Tile_VecID {}, Not Found", glob);
        return 0;
    }
}


#[derive(serde_derive::Deserialize, Debug)]
pub struct ImportedTile {
    id: u32,
    class: String,
    properties: Vec<ImpProperties>,
}

impl ImportedTile {
    pub fn id(&self) -> usize {return self.id as usize}
    //checks through the vetor of properties for flag defined as string
    pub fn flag_val(&self, f: String) -> bool {
        //for each property for importedTiles
        for i in 0..self.properties.len() {
            if self.properties[i].name == f {
                return self.properties[i].value;
            }
        }
        println!("Json Import Tile Error - TileID: {} No {:?} flag found", self.id, f);
        return false;
    }
}


#[derive(serde_derive::Deserialize, Debug)]
pub struct ImpProperties {
    pub name: String,
    pub value: bool,
}

pub fn read_json_from_file<P: AsRef<Path>>(path: P)
    -> Result<ImportedRouteJson, Box<dyn Error>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

#[cfg(test)]
mod tests {
    use crate::world::json_importer::read_json_from_file;

    #[test]
    fn test_json_import() {

        let u = read_json_from_file("resources/routes/titanium_town1_V02.json").unwrap();
        println!("does printing not work: {:?}", u);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



