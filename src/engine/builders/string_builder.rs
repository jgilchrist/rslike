use engine::{Map, Tile};
use engine::builders::{MapBuilder, MapBuildError, MapBuildResult};

use std::iter::{IteratorExt};

pub struct MapFromString {
    s: String,
}

impl MapFromString {

    pub fn new(s: &str) -> MapFromString {
        MapFromString { s: s.to_string() }
    }

    fn build_line(&self, line: &str) -> Vec<Tile> {
        line.chars().map(|char| self.build_cell(char)).collect()
    }

    fn build_cell(&self, cell: char) -> Tile {
        match cell {
            ' ' => Tile::Empty,
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            _ => { panic!("Invalid character in level string: {}", cell); }
        }
    }
}

impl MapBuilder for MapFromString {
    fn build(&self) -> MapBuildResult {
        let lines : Vec<_> = self.s.split('\n').collect();

        if !lines.iter().all(|x| x.len() == lines[0].len()) { return Err(MapBuildError::new("Different length lines")) };

        let tiles: Vec<Vec<Tile>> = lines.iter().map(|&line| self.build_line(line)).collect();

        Ok(Map::new(tiles))
    }
}
