use engine::Tile;
use engine::Tile::{EmptyTile, FloorTile, WallTile};

pub struct Map {
    tiles: Vec<Vec<Tile>>
}

impl Map {

    fn new() -> Map {
        let tiles = vec![
            vec![EmptyTile, EmptyTile, EmptyTile],
            vec![WallTile, WallTile, WallTile],
            vec![WallTile, FloorTile, WallTile],
            vec![WallTile, WallTile, WallTile],
            vec![EmptyTile, EmptyTile, EmptyTile],
        ];

        Map {
            tiles: tiles
        }
    }

}
