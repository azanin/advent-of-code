use std::fs::read_to_string;
use std::io::Error;
use std::num::ParseIntError;
use std::convert::TryInto;

#[derive(Debug)]
enum CliError {
    Io(Error),
    Parse(ParseIntError),
}

fn main() {
    let input_values: Result<Vec<i32>, CliError> =
        read_to_string("input.txt").map_err(CliError::Io)
            .and_then(|s| s.lines().map(|l| l.parse::<i32>().map_err(CliError::Parse)).collect());

    let results: Result<Vec<i32>, CliError> = input_values.map(|vs| vs.iter().flat_map(|&a| {
        vs.iter().flat_map(move |&b| if a + b == 2020 { vec![a * b] } else { Vec::new() })
    }).collect());


    println!("{:?}", results);
}
