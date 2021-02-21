mod cell;
mod map;

use cell::Cell;
use std::fs::read_to_string;
use std::error::Error;
use crate::map::{Map, Position, Strategy};


fn main() -> Result<(), Box<dyn Error>> {

    let input   =
        read_to_string("input.txt")
            .map(|s| s.lines().map(|l| Cell::parse(l.to_string())).collect())
            .map(|cells| Map::create(cells))
            .map(|map| map.count_tree(Position::create(0, 0), Strategy::create(3, 1)))?;

    Ok(println!("Result {}", input))
}
