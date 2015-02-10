use engine::{Tile, Map};
use util::units::Size;

use std::old_io;

pub trait IntoMap {
    fn as_map(self) -> MapBuildResult;
}

impl IntoMap for Vec<Vec<Tile>> {
    fn as_map(self) -> MapBuildResult {
        // TODO: check dimensions are the same
        let size = Size::new(self.len() as i32, self[0].len() as i32);

        Ok(Map {
            tiles: self,
            size: size,
        })
    }
}

fn build_line(l: &&str) -> Vec<Tile> {
    l.chars().map(|c| Tile::from_char(c)).collect()
}

impl IntoMap for String {
    fn as_map(self) -> MapBuildResult {
        let lines: Vec<&str> = self.split('\n').collect();

        if !lines.iter().all(|x| x.len() == lines[0].len()) { return Err("Different length lines") };

        let tiles: Vec<Vec<Tile>> = lines.iter().map(build_line).collect();

        Ok(Map::new(tiles))
    }
}

impl IntoMap for Path {
    fn as_map(self) -> MapBuildResult {
        let mut level_file = old_io::File::open(&self);
        let level_string = level_file.read_to_string().ok().expect("Failed to read level file");

        Ok(Map::new(level_string))
    }
}

pub type MapBuildResult = Result<Map, &'static str>;
