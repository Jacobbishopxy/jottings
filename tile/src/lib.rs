//! Tile
//!
//! https://github.com/dtolnay/ref-cast

#![allow(dead_code)]

use std::ops::Index;

use ref_cast::RefCast;

const MAP_WIDTH: usize = 4;

#[derive(Debug)]
struct Tile(u8);

#[derive(Debug)]
struct TileMap {
    storage: Vec<Tile>,
}

#[derive(Debug, RefCast)]
#[repr(transparent)]
struct Strided([Tile]);

// Implement `tilemap[x][y]` as `tilemap[x..][y * MAP_WIDTH]`.
impl Index<usize> for TileMap {
    type Output = Strided;

    fn index(&self, x: usize) -> &Self::Output {
        assert!(x < MAP_WIDTH);
        Strided::ref_cast(&self.storage[x..])
    }
}

impl Index<usize> for Strided {
    type Output = Tile;

    fn index(&self, y: usize) -> &Self::Output {
        &self.0[y * MAP_WIDTH]
    }
}

#[test]
fn tile_index_success() {
    let tm = TileMap {
        storage: vec![
            Tile(10),
            Tile(11),
            Tile(12),
            Tile(13),
            Tile(14),
            Tile(15),
            Tile(16),
            Tile(17),
        ],
    };

    println!("{:?}", tm[2][1]);
}
