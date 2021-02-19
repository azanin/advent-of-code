mod cell;
mod map;

use cell::Cell;
use std::fs::read_to_string;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    let input = read_to_string("input.txt").map(|s| Cell::parse(s));


    Ok(println!("wow"))
}
