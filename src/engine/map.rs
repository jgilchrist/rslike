use engine::Tile;
use util::FromChar;
use util::units::{Point, Size};

use std::path::Path;
use std::fs::File;
use std::io::Read;

/// A map.
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub size: Size,
    pub starting_position: Point,
}

impl Map {

    /// Creates a new map from a string.
    pub fn from_string(s: String) -> Map {
        let lines: Vec<&str> = s.split('\n')
                                   .filter(|l| !l.is_empty())
                                   .collect();

        let expected_line_length = lines.first().expect("Map string contained no lines").len();

        if !lines.iter().all(|x| x.len() == expected_line_length) {
            panic!("Different length lines in level string")
        }

        let tiles: Vec<Vec<Tile>> = lines.iter()
                                         .map(|l| l.chars()
                                                   .map(|c| Tile::from_char(c))
                                                   .collect())
                                         .collect();

        let mut starting_position = None;

        for (y, row_vec) in lines.iter().enumerate() {
            for (x, tile) in row_vec.chars().enumerate() {
                if tile == '@' {
                    starting_position = Some(Point::new(x as i32, y as i32));
                }
            }
        }

        let starting_position = starting_position.expect("Map string did not contain a starting position, '@'");

        let size = Size::new(tiles.len() as i32, tiles[0].len() as i32);

        Map {
            tiles: tiles,
            size: size,
            starting_position: starting_position,
        }
    }

    pub fn from_file<T>(path: T) -> Map where T: AsRef<Path> {
        let mut level_file = File::open(path).ok().expect("Could not find level file");

        let mut level_string = String::new();
        level_file.read_to_string(&mut level_string).ok().expect("Could not read from level file");

        Map::from_string(level_string)
    }

    pub fn at(&self, loc: Point) -> Tile {
        self.tiles[loc.y as usize][loc.x as usize]
    }

    pub fn is_walkable(&self, loc: Point) -> bool {
        self.at(loc).is_walkable()
    }

    pub fn set_tile(&mut self, loc: Point, tile: Tile) {
        self.tiles[loc.y as usize][loc.x as usize] = tile;
    }

    pub fn height(&self) -> i32 {
        self.tiles.len() as i32
    }

    pub fn width(&self) -> i32 {
        self.tiles[0].len() as i32
    }

}
