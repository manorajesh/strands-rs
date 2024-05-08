use std::iter::zip;

use rand::Rng;

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Cell {
    letter: char,
    x: usize,
    y: usize,
}

impl Cell {
    fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

struct Board(Vec<Vec<Cell>>);

impl Board {
    fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut board: Vec<Vec<Cell>> = Vec::new();
        for y in 0..num_rows {
            board.push(Vec::new());
            for x in 0..num_cols {
                board[y].push(Cell {
                    letter: char::default(),
                    x,
                    y,
                });
            }
        }

        Board(board)
    }

    fn print(&self) {
        for row in &self.0 {
            for cell in row {
                print!("{} ", cell.letter);
            }
            println!();
        }
    }

    fn valid_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        if x < 0 || (x as usize) > self.0.len() {
            return None;
        }

        // assumes rectangular board and there is at least one row
        if y < 0 || (y as usize) > self.0[0].len() {
            return None;
        }

        if let Some(row) = self.0.get(y as usize) {
            if let Some(cell) = row.get(x as usize) {
                if cell.letter == char::default() {
                    return Some(cell);
                }
            }
        }

        None
    }

    fn get_next_random_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let mut rng = rand::thread_rng();

        for _ in 0..9 {
            let x_offset: i32 = rng.gen_range(-1..=1);
            let y_offset: i32 = rng.gen_range(-1..=1);

            let x = (x as i32) + x_offset;
            let y = (y as i32) + y_offset;

            if let Some(cell) = self.valid_cell(x, y) {
                return Some(cell);
            }
        }

        panic!("No valid cell found");
    }

    fn place_word(&mut self, word: &str) {
        let mut current_cell = self.0
            .iter()
            .flatten()
            .find(|&&cell| cell.letter == char::default())
            .unwrap();

        let mut positions: Vec<(usize, usize)> = Vec::new();

        for char in word.chars() {
            while positions.contains(&current_cell.coords()) {
                current_cell = self.get_next_random_cell(current_cell.x, current_cell.y).unwrap();
            }

            let new_cell = Cell {
                letter: char,
                ..*current_cell
            };

            positions.push(new_cell.coords());
        }

        for (coords, letter) in zip(positions, word.chars()) {
            let (x, y) = coords;
            self.0[y][x].letter = letter;
            self.print();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

fn main() {
    let words = ["invite", "letter", "snailmail", "magazine", "postcard", "bill", "package"];
    let mut board = Board::new(8, 6);

    board.print();

    let mut sum = 0;
    for word in words.iter() {
        sum += word.len();
    }
    println!("Sum of word lengths: {}", sum);

    for word in words.iter() {
        board.place_word(word);
        board.print();
    }

    board.print();
}
