use engine::{Tile, Map};
use util::{FirstLast, FromChar};
use util::units::Size;

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

impl IntoMap for String {
    fn as_map(self) -> MapBuildResult {
        let lines: Vec<&str> = self.split('\n')
                                   .filter(|l| !l.is_empty())
                                   .collect();

        let expected_line_length = lines.first().len();

        if !lines.iter().all(|x| x.len() == expected_line_length) {
            return Err("Different length lines")
        }

        let tiles: Vec<Vec<Tile>> = lines.iter()
                                         .map(|l| l.chars()
                                                   .map(|c| Tile::from_char(c))
                                                   .collect())
                                         .collect();

        Ok(Map::new(tiles))
    }
}

pub type MapBuildResult = Result<Map, &'static str>;
