use crate::cell::Cell::{Tree, Free};

#[derive(PartialEq, Debug)]
pub enum Cell {
    Tree,
    Free
}

impl Cell {

    fn parse_cell(char: char) -> Cell {
        match char {
            '#' => Tree,
            '.' => Free,
            _ => panic!("unexpected check the input file")
        }
    }

    pub fn parse(line: String) -> Vec<Cell> {
        line.chars().map(|c| Self::parse_cell(c)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_tree() {
        let input = '#';
        let actual = Cell::parse_cell(input);
        let expected_result = Tree;
        assert_eq!(actual, expected_result);
    }

    #[test]
    fn parse_free() {
        let input = '.';
        let actual = Cell::parse_cell(input);
        let expected_result = Free;
        assert_eq!(actual, expected_result);
    }

    #[test]
    fn parse_cells() {
        let input = String::from(".#.");
        let actual = Cell::parse(input);
        let expected_result = vec![Free, Tree, Free];
        assert_eq!(actual, expected_result);
    }
}

