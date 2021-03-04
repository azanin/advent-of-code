#![feature(str_split_once)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use regex::Regex;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules: Vec<(&str, fn(&str) -> bool)> =
        vec![("byr", |str: &str| (1920..2003).contains(&str.parse::<i32>().unwrap())),
             ("iyr", |str: &str| (2010..2021).contains(&str.parse::<i32>().unwrap())),
             ("eyr", |str: &str| (2020..2031).contains(&str.parse::<i32>().unwrap())),
             ("hgt", |str: &str| {
                 return if str.ends_with("cm") && str.len() == 5 {
                     (150..194).contains(&str[0..3].parse::<i32>().unwrap())
                 } else if str.ends_with("in") && str.len() == 4 {
                     (59..77).contains(&str[0..2].parse::<i32>().unwrap())
                 } else { false };
             }),
             ("hcl", |str: &str| str.starts_with('#') && str.len() == 7 && str[1..].chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))),
             ("ecl", |str: &str| vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|&v| v == str)),
             ("pid", |str: &str| str.len() == 9 && str.chars().all(|c| c.is_digit(10)))
        ];

    let file = read_to_string("input.txt")?;
    let passports_raw: Vec<&str> = parse_file_string(file.as_str())?;
    let passports_as_maps: Vec<HashMap<&str, &str>> = passports_raw.iter().map(|&line| parse(line)).collect();

    let valid1 = passports_as_maps.iter().filter(|&pass| is_valid(pass, &rules)).count();
    let valid2 = passports_as_maps.iter().filter(|&pass| is_valid2(pass, &rules)).count();

    return Ok(println!("{} {}", valid1, valid2));
}

fn parse_file_string(file: &str) -> Result<Vec<&str>, regex::Error> {
    let regex = Regex::new(r"\s+\n+")?;
    return Ok(regex.split(file).collect());
}

fn parse(passport_raw: &str) -> HashMap<&str, &str> {
    let fields: Vec<&str> = passport_raw.split(&['\n', ' '][..]).collect();
    let b = fields.iter().filter(|&x| !x.is_empty()).map(|&field| field.split_once(':').unwrap()).collect::<HashMap<_, _>>();
    return b;
}

fn is_valid(passport: &HashMap<&str, &str>, rules: &Vec<(&str, fn(&str) -> bool)>) -> bool {
    return rules.iter().all(|&key| passport.contains_key(key.0));
}

fn is_valid2(passport: &HashMap<&str, &str>, rules: &Vec<(&str, fn(&str) -> bool)>) -> bool {
    return rules.iter().all(|&k| {
        let key = k.0;
        let predicate = k.1;
        return passport.get(key).filter(|&v| predicate(v)).is_some();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_string_test() {
        let input = "iyr:2015 cid:189 ecl:oth byr:1947 hcl:#6c4ab1 eyr:2026\nhgt:174cm\npid:526744288\n    \npid:688706448 iyr:2017 hgt:162cm cid:174 ecl:grn byr:1943 hcl:#808e9e eyr:2025";

        let actual = parse_file_string(input);
        let expected = Ok(vec![
            "iyr:2015 cid:189 ecl:oth byr:1947 hcl:#6c4ab1 eyr:2026\nhgt:174cm\npid:526744288",
            "pid:688706448 iyr:2017 hgt:162cm cid:174 ecl:grn byr:1943 hcl:#808e9e eyr:2025"
        ]);

        assert_eq!(actual, expected)
    }


    #[test]
    fn is_valid2_test() {
        let input: HashMap<&str, &str> = [
            ("key1", "koala"),
            ("key2", "another value")
        ].iter().cloned().collect();

        let rules: Vec<(&str, fn(&str) -> bool)> = vec![("key1", |str: &str| str.contains("k"))];


        let valid2 = is_valid2(&input, &rules);
        assert!(valid2)
    }
}
