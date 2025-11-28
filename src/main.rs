use std::{collections::HashMap, num::NonZeroU8};

use clap::Parser;

use soduk::Board;

#[derive(Parser)]
struct Cli {
    /// triplets of row (1 .. 9), column and value
    /// example `3 4 5` means (row3 col4) = 5
    values: Vec<u8>,
}

fn main() {
    let cli = Cli::parse();
    assert!(
        cli.values.len() % 3 == 0,
        "expected values.len() to be divisiable by 3, got \t\n{:?}",
        cli.values
    );

    if let Some(min) = cli.values.iter().min() {
        assert!(*min > 0, "1 is smallest allowed value");
    };

    if let Some(max) = cli.values.iter().max() {
        assert!(*max <= 9, "9 is biggest allowed value");
    };

    let mut board = Board::default();
    for (row, col, number) in cli
        .values
        .chunks_exact(3)
        .map(|a| (a[0], a[1], NonZeroU8::new(a[2])))
    {
        board.set(row - 1, col - 1, number);
    }

    println!("input: \n{board}");
    let solution = board.solve();
    match solution {
        Some(s) => {
            println!("found solution: \n{s}");
        }
        None => panic!("no solution"),
    }
}
