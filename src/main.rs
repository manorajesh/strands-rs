struct Board(Vec<Vec<char>>);

impl Board {
    fn new(num_rows: usize, num_cols: usize) -> Self {
        Board(vec![vec![' '; num_cols]; num_rows])
    }

    fn print(&self) {
        for row in &self.0 {
            for cell in row {
                print!("{cell} ");
            }
            println!();
        }
    }
}

fn main() {
    let words = ["invite", "letter", "snailmail", "magazine", "postcard", "bill", "package"];
    let mut board = Board::new(8, 6);

    board.print()
}
