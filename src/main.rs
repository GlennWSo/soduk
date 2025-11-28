use std::{
    fmt::{Display, Formatter},
    num::NonZeroU8,
};

type Onum = Option<NonZeroU8>;

type State = [[Onum; 9]; 9];

#[derive(Default, Copy, Clone, Debug)]
struct Board(State);

fn unique(data: &[Onum]) -> bool {
    for n in (1..=9).map(|n| NonZeroU8::new(n)) {
        if data.iter().filter(|v| **v == n).count() > 1 {
            return false;
        }
    }
    true
}

impl Board {
    fn square(&self, row: usize, col: usize) -> [Onum; 9] {
        let rows = row..(row + 3);
        let data: Vec<_> = rows
            .flat_map(|row| {
                let row = self.0[row];
                row.into_iter().skip(col).take(3)
            })
            .collect();
        data.try_into().unwrap()
    }
    fn check_boxes(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                let data = self.square(row * 3, col * 3);
                if !unique(&data) {
                    return false;
                }
            }
        }
        true
    }

    fn check_column(&self, column: usize) -> bool {
        let cols: Box<[_]> = (0..9).map(|row| self.0[row][column]).collect();
        unique(&cols)
    }
    fn check_columns(&self) -> bool {
        (0..9).all(|i| self.check_column(i))
    }

    fn check_row(&self, row: usize) -> bool {
        unique(&self.0[row])
    }
    fn check_rows(&self) -> bool {
        (0..9).all(|i| self.check_row(i))
    }
    fn check(&self) -> bool {
        self.check_rows() && self.check_columns() && self.check_boxes()
    }
    fn next_pos(&self) -> Option<usize> {
        self.0
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, num)| match num {
                Some(_) => None,
                None => Some(idx),
            })
            .next()
    }
    fn set_pos(&mut self, pos: usize, n: Onum) {
        let row = pos / 9;
        let col = pos % 9;
        self.0[row][col] = n;
    }

    fn solve(mut self) -> Option<Self> {
        if !self.check() {
            return None;
        }
        let Some(pos) = self.next_pos() else {
            return Some(self);
        };
        for n in (1..=9).map(|i| NonZeroU8::new(i)) {
            self.set_pos(pos, n);
            if let Some(solution) = self.solve() {
                return Some(solution);
            }
        }
        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let div = "-------------------------------------";
        for row in self.0 {
            writeln!(f, "{div}")?;
            for num in row {
                let x = match num {
                    Some(v) => format!("{v}"),
                    None => "-".into(),
                };
                write!(f, "| {x} ")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "{div}")
    }
}

fn main() {
    let board = Board::default();
    println!("{board}");
    let solution = board.solve();
    match solution {
        Some(s) => {
            println!("{s}");
            println!("{:?}", s.square(0, 0));
        }
        None => panic!("no solution"),
    }
}
