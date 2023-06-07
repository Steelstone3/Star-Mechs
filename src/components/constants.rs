use std::ops::Range;

#[allow(dead_code)]
pub const TILE_SIZE: f32 = 32.0;
#[allow(dead_code)]
pub const MAP_TILES: Range<i32> = -NUMBER_OF_TILES..NUMBER_OF_TILES;
#[allow(dead_code)]
pub const MAP_SIZE: f32 = TILE_SIZE * NUMBER_OF_TILES as f32;
#[allow(dead_code)]
const NUMBER_OF_TILES: i32 = 64;
