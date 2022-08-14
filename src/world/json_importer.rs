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
pub struct ImportedRouteJson {
    compressionlevel: i8,
    layers: Vec<ImportedLayers>,
    tileheight: u32,
    tilewidth: u32,
    tilesets: Vec<ImportedTilesets>
}
impl ImportedRouteJson {
    pub fn layer(&self, u: usize) -> &ImportedLayers {return &self.layers[u]}
    pub fn num_tilesets(&self) -> usize {return self.tilesets.len()}
    pub fn tilesets(&self) -> &Vec<ImportedTilesets> {return &self.tilesets}
}
#[derive(serde_derive::Deserialize, Debug)]
pub struct ImportedLayers {
    data: Vec<i32>,
    height: u32,
    width: u32,
}
impl ImportedLayers {
    pub fn data_by_ref(&self) -> &Vec<i32> {return &self.data}
    pub fn data_len(&self) -> usize {return self.data.len()}
}


#[derive(serde_derive::Deserialize, Debug)]
pub struct ImportedTilesets {
    firstgid: u32,
    pub tiles: Vec<ImportedTile>,
}
impl ImportedTilesets {
    pub fn num_tiles_in_imported_sheet(&self) -> usize {return self.tiles.len()}
    pub fn firstgid(&self) -> u32 {self.firstgid}
    pub fn is_tile_walkable(&self, i: usize) -> bool {self.tiles[i].flag_val("Walkable".to_string())}
    pub fn is_tile_spawner(&self, i: usize) -> bool {self.tiles[i].flag_val("Spawn".to_string())}
    pub fn get_tile_class(&self, i: usize) -> TileClass {
        
        if i > self.tiles.len() {println!("ERROR HERE I: {} > Tile Length {}", i, self.tiles.len())}

        let str = self.tiles[i].class.clone();
        if str == "Normal" {return TileClass::Normal;}
        if str == "Jump" {return TileClass::Jump;}
        if str == "Door" {return TileClass::Door;}
        
        println!("Jaon Import Tile Error - Found unknown class: {} at ID: {}", str, i);
        return TileClass::Normal;
    }
}


#[derive(serde_derive::Deserialize, Debug)]
pub struct ImportedTile {
    id: u32,
    class: String,
    properties: Vec<ImpProperties>,
}

impl ImportedTile {
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


impl ImportedLayers {
    pub fn data(&self) -> Vec<i32> {return self.data.clone()}
    pub fn width(&self) -> u32 {return self.width}
    pub fn height(&self) -> u32 {return self.height}
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



