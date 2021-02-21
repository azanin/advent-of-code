use crate::cell::Cell;

pub struct Map {
    cells: Vec<Vec<Cell>>
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    column: usize,
    row: usize,
}

impl Position {
    pub fn create(column: i32, row: i32) -> Position { Position { column: column as usize, row: row as usize } }
}

#[derive(Clone, Copy)]
pub struct Strategy {
    right: usize,
    down: usize,
}

impl Strategy {
    pub fn create(right: i32, down: i32) -> Strategy { Strategy { right: right as usize, down: down as usize } }
}

impl Map {
    fn dimension(&self) -> (usize, usize) {
        (self.cells.len(), self.cells.first().map(|x| x.len()).unwrap_or(0))
    }


    fn next_position(&self, actual_position: Position, strategy: Strategy) -> Option<Position> {
        return if (actual_position.row + 1) as usize == self.dimension().0 {
            None
        } else {
            Some(Position { row: actual_position.row + strategy.down, column: (actual_position.column + strategy.right) % self.dimension().1 })
        };
    }

    fn count(&self, position: Position) -> i32 {
        let cell = self.cells.get(position.row as usize)
            .and_then(|vec: &Vec<Cell>| vec.get(position.column as usize));

        match cell {
            Some(Cell::Tree) => 1,
            Some(Cell::Free) => 0,
            _ => 0
        }
    }

    pub fn count_tree(&self, start_position: Position, strategy: Strategy) -> i32 {
        self.count_tree_rec(start_position, strategy, 0)
    }

    fn count_tree_rec(&self, actual_position: Position, strategy: Strategy, counter: i32) -> i32 {
        let next_position = self.next_position(actual_position, strategy);

        match next_position {
            Some(position) => self.count_tree_rec(position, strategy, self.count(position) + counter),
            None => counter
        }
    }

    pub fn create(cells: Vec<Vec<Cell>>) -> Map { Map { cells } }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cell::Cell::{Tree, Free};

    #[test]
    fn dimension() {
        let input = Map { cells: vec![vec![Tree, Free], vec![Free, Tree]] };
        let actual = input.dimension();
        let expected = (2 as usize, 2 as usize);

        assert_eq!(actual, expected);
    }

    #[test]
    fn next_position_3_1() {
        let input = Map {
            cells: vec![vec![Tree, Free, Tree, Free, Tree, Free, Tree, Free],
                        vec![Free, Tree, Free, Tree, Free, Tree, Free, Tree]]
        };

        let start_position = Position { column: 0, row: 0 };
        let strategy = Strategy { right: 3, down: 1 };

        let expected = Some(Position { column: 3, row: 1 });

        let actual = input.next_position(start_position, strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn next_position_none() {
        let input = Map {
            cells: vec![vec![Tree, Free, Tree, Free, Tree, Free, Tree, Free],
                        vec![Free, Tree, Free, Tree, Free, Tree, Free, Tree]]
        };

        let start_position = Position { column: 5, row: 1 };
        let strategy = Strategy { right: 3, down: 1 };

        let expected: Option<Position> = None;

        let actual = input.next_position(start_position, strategy);

        assert_eq!(actual, expected);
    }
}