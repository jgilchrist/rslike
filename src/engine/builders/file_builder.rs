use engine::builders::{MapBuilder, MapBuildResult, MapFromString};

use std::io;

pub struct MapFromFile {
    p: Path,
}

impl MapFromFile {

    pub fn new(p: Path) -> MapFromFile {
        MapFromFile { p: p }
    }

}

impl MapBuilder for MapFromFile {
    fn build(&self) -> MapBuildResult {
        let mut level_file = io::File::open(&self.p);
        let level_string = level_file.read_to_string().ok().expect("Failed to read level file");

        MapFromString::new(level_string.as_slice()).build()
    }
}
