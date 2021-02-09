use std::fs::read_to_string;
use std::ops::Deref;
use std::error::Error;

#[derive(Debug)]
struct Password {
    at_least: usize,
    at_most: usize,
    character: char,
    value: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let passwords =
        read_to_string("input.txt")?
            .lines()
            .map(|l| parse(l))
            .collect::<Result<Vec<Password>, Box<dyn Error>>>()?;

    let result = passwords.iter()
        .filter(|&password| {
            let occurrence =
                password.value
                    .chars()
                    .filter(|c| c.eq(&password.character))
                    .count();
            
            occurrence >= password.at_least && occurrence <= password.at_most
        })
        .count();

    Ok(println!("{:?}", result))
}

fn parse(line: &str) -> Result<Password, Box<dyn Error>> {
    let vec: Vec<&str> = line.split([' ', '-', ':'].as_ref()).collect();

    let at_least = vec[0].parse::<usize>()?;
    let at_most = vec[1].parse::<usize>()?;
    let character = vec[2].parse::<char>()?;
    let password_value = vec[4];

    Ok(Password {
        at_least,
        at_most,
        character,
        value: String::from(password_value),
    })
}