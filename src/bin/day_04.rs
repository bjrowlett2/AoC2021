mod aoc;

#[derive(Clone)]
struct Cell {
    value: i64,
    marked: bool,
}

impl Cell {
    fn new(value: i64) -> Cell {
        return Cell {
            value: value,
            marked: false,
        };
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn new() -> Board {
        return Board {
            cells: vec![],
        };
    }

    fn mark(&mut self, called: i64) {
        for cell in &mut self.cells {
            if cell.value == called {
                cell.marked = true;
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row in 0..5 {
            if self.is_winning_row(row) {
                return true;
            }
        }

        for column in 0..5 {
            if self.is_winning_column(column) {
                return true;
            }
        }

        return false;
    }

    fn is_winning_row(&self, row: usize) -> bool {
        for column in 0..5 {
            let index = row * 5 + column;
            if !self.cells[index].marked {
                return false;
            }
        }

        return true;
    }

    fn is_winning_column(&self, column: usize) -> bool {
        for row in 0..5 {
            let index = row * 5 + column;
            if !self.cells[index].marked {
                return false;
            }
        }

        return true;
    }

    fn score(&self, just_called: i64) -> i64 {
        let mut sum_unmarked = 0;
        for cell in &self.cells {
            if !cell.marked {
                sum_unmarked += cell.value;
            }
        }

        return sum_unmarked * just_called;
    }
}

#[derive(Clone)]
struct Day04 {
    called: Vec<i64>,
    boards: Vec<Board>,
}

fn main() {
    let mut day = Day04 {
        called: vec![],
        boards: vec![],
    };

    let lines = aoc::lines("inputs/day_04.txt");

    for called in lines[0].split(",") {
        match called.parse::<i64>() {
            Ok(value) => day.called.push(value),
            Err(reason) => panic!("String::parse failed: {}", reason),
        };
    }

    let mut i = 2;
    while i < lines.len() {
        let mut board = Board::new();

        for j in 0..5 {
            let row = &lines[i + j];
            for cell in row.split(" ") {
                if !cell.is_empty() {
                    match cell.parse::<i64>() {
                        Ok(value) => board.cells.push(Cell::new(value)),
                        Err(reason) => panic!("String::parse failed: {}", reason),
                    };
                }
            }
        }

        day.boards.push(board);

        i += 6; // 5 lines for the board + 1 blank line.
    }

    match solve_part_1(&day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    }

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    }
}

fn solve_part_1(day: &Day04) -> Result<i64, String> {
    let mut clone = day.clone();

    for called in clone.called {
        for board in &mut clone.boards {
            board.mark(called);

            if board.is_winner() {
                return Ok(board.score(called));
            }
        }
    }

    return Err(String::from("No solution found"));
}

fn solve_part_2(day: &Day04) -> Result<i64, String> {
    let mut clone = day.clone();

    let mut num_winners = 0;
    let num_boards = clone.boards.len();

    for called in clone.called {
        for board in &mut clone.boards {
            if !board.is_winner() {
                board.mark(called);

                if board.is_winner() {
                    num_winners += 1;
                    if num_winners == num_boards {
                        return Ok(board.score(called));
                    }
                }
            }
        }
    }

    return Err(String::from("No solution found"));
}
