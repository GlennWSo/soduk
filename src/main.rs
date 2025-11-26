use std::{
    fmt::{Display, Formatter},
    num::NonZeroU8,
};

type Onum = Option<NonZeroU8>;

type State = [[Onum; 9]; 9];

#[derive(Default)]
struct Board(State);

impl Board {
    fn check_row(&self, i: usize) -> bool {
        todo!()
    }
    fn check_rows(&self) -> bool {
        (0..9).all(|i| self.check_row(i))
    }
    fn check(&self) -> bool {
        todo!()
    }
    fn next_pos(&self) -> Option<usize> {
        todo!()
    }
    fn set_pos(&mut self, pos: usize, n: Onum) {
        todo!()
    }

    fn solve(mut self) -> Option<Self> {
        let Some(pos) = self.next_pos() else {
            if self.check() {
                return Some(self);
            }
            return None;
        };
        let solved = (0..9).any(|n| {
            self.set_pos(pos, NonZeroU8::new(n));
            self.check()
        });
        if solved { Some(self) } else { self.solve() }
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
    println!("{board}")
}
