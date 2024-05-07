use rand::Rng;

#[derive(Default, Clone, Copy, Debug)]
struct Cell {
    letter: char,
    x: usize,
    y: usize,
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
                if cell.letter != char::default() {
                    return Some(cell);
                }
            }
        }

        None
    }

    fn mutate_cell_letter(&mut self, x: usize, y: usize, letter: char) {
        self.0[y][x].letter = letter;
    }

    fn place_word(&mut self, word: &str) {
        let mut rng = rand::thread_rng();

        let mut current_cell = self.0
            .iter()
            .flatten()
            .find(|&&cell| cell.letter == char::default())
            .unwrap();

        for char in word.chars() {
            loop {
                let x_offset: i32 = rng.gen_range(-1..=1);
                let y_offset: i32 = rng.gen_range(-1..=1);

                let x = (current_cell.x as i32) + x_offset;
                let y = (current_cell.y as i32) + y_offset;

                if let Some(cell) = self.valid_cell(x, y) {
                    self.mutate_cell_letter(x, y, cel);
                    current_cell = cell;
                    break;
                }
            }
        }
    }
}

fn main() {
    let words = ["invite", "letter", "snailmail", "magazine", "postcard", "bill", "package"];
    let mut board = Board::new(8, 6);

    board.print();

    board.place_word(words[0]);

    board.print();
}
