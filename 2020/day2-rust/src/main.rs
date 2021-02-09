use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Password {
    at_least: i32,
    at_most: i32,
    character: char,
    value: String,
}

impl Password {

    fn is_valid(&self) -> bool {
        let occurrence =
            self.value
                .chars()
                .filter(|c| *c == self.character)
                .count();

        occurrence >= self.at_least as usize && occurrence <= self.at_most as usize
    }

    fn is_valid2(&self) -> bool {
        let char1 = self.value.chars().nth((self.at_least - 1) as usize);
        let char2 = self.value.chars().nth((self.at_most - 1) as usize);

        return (char1 == Some(self.character)) != (char2 == Some(self.character));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let passwords =
        read_to_string("input.txt")?
            .lines()
            .map(|l| parse(l))
            .collect::<Result<Vec<Password>, Box<dyn Error>>>()?;

    let result1 = passwords.iter()
        .filter(|&password| password.is_valid())
        .count();

    let result2 = passwords.iter()
        .filter(|&password| password.is_valid2())
        .count();


    Ok(println!("{:?} and {:?}", result1, result2))
}

fn parse(line: &str) -> Result<Password, Box<dyn Error>> {
    let vec: Vec<&str> = line.split([' ', '-', ':'].as_ref()).collect();

    let at_least = vec[0].parse::<i32>()?;
    let at_most = vec[1].parse::<i32>()?;
    let character = vec[2].parse::<char>()?;
    let password_value = vec[4];

    Ok(Password {
        at_least,
        at_most,
        character,
        value: String::from(password_value),
    })
}