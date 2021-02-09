use std::fs::read_to_string;
use std::ops::Deref;

#[derive(Debug)]
struct Password {
    at_least: usize,
    at_most: usize,
    character: char,
    value: String
}

fn main() {
    let input_string = read_to_string("input.txt");

    let res: Result<Vec<Password>, std::io::Error> =
        input_string.map(|raw_string| raw_string.lines().map(|l| parse(l)).collect());

    let result = res.map(|passwords| passwords.iter().filter(|&p| {
        let occurrence = p.value.chars().filter(|c| c.eq(&p.character)).count();
        occurrence >= p.at_least && occurrence <= p.at_most
    }).count());


    println!("{:?}", result);

}

fn parse(line: &str) -> Password {
    let vec: Vec<&str> =
        line.split([' ', '-', ':'].as_ref()).collect();

    let at_least = vec[0].parse::<usize>().unwrap();
    let at_most = vec[1].parse::<usize>().unwrap();
    let character = vec[2].parse::<char>().unwrap();
    let password_value = vec[4];

    Password {
        at_least,
        at_most,
        character,
        value: String::from(password_value)
    }
}