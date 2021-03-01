use std::fs::read_to_string;
use regex::{Regex};
use std::error::Error;
use std::iter::Map;
use std::collections::HashMap;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let vec = parse_file_string(input.as_str())?;

    let maps: Vec<HashMap<&str, &str>> = vec.iter().map(|&line| parse(line)).collect();

    for el in maps {
        for passport in el {
            println!("<{} - {}>", passport.0, passport.1)
        }
    }

    return Ok(());
}

fn parse_file_string(file: &str) -> Result<Vec<&str>, regex::Error> {
    let regex = Regex::new(r"\s+\n+")?;
    return Ok(regex.split(file).collect());
}

fn parse(passport_raw: &str) -> HashMap<&str, &str> {
    let fields: Vec<&str> = passport_raw.split(&['\n', ' '][..]).collect();
    let b: Vec<Vec<&str>> = fields.iter().map(|&field| field.split(':').collect()).collect();
    return build_map(b);
}

fn build_map(vec: Vec<Vec<&str>>) -> HashMap<&str, &str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    for pair in vec {
        map.insert(pair.first().unwrap(), pair.last().unwrap());
    }
    return map;
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
}
