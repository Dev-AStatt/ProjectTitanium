extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use std::{
    fs::File,
    io::BufReader,
    path::Path,
    error::Error,
};


#[derive(serde_derive::Deserialize, Debug)]
pub struct ImportedRouteJson {
    compressionlevel: i8,
    layers: Vec<ImportedLayers>,
    tileheight: u32,
    tilewidth: u32,
}
impl ImportedRouteJson {
    pub fn data_layer(&self, u: usize) -> Vec<i32> {return self.layers[u].data()}
    pub fn layer(&self, u: usize) -> &ImportedLayers {return &self.layers[u]}
}


#[derive(serde_derive::Deserialize, Debug)]
pub struct ImportedLayers {
    data: Vec<i32>,
    height: u32,
    width: u32,
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
    use crate::world::route::read_json_from_file;

    #[test]
    fn test_json_import() {

        let u = read_json_from_file("Titanium_Route1.json").unwrap();
        println!("{:?}", u);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



