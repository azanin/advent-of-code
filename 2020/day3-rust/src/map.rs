use crate::cell::Cell;

struct Map {
    cells: Vec<Vec<Cell>>
}

struct Position {
    column: i32,
    row: i32,
}

struct Strategy {
    right: i32,
    down: i32,
}

impl Map {
    fn dimension(&self) -> (usize, usize) {
        (self.cells.len(), self.cells.first().map(|x| x.len()).unwrap_or(0))
    }


    fn next_position(&self, actual_position: Position, strategy: Strategy) -> Option<Position> {
        if (actual_position.column + 1) as usize == self.dimension().0 {
            return None;
        } else {
            return Some(Position { column: actual_position.column + strategy.down, row: actual_position.row + strategy.right });
        }
    }
}