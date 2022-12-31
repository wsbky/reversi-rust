use crate::models::{board::Board, color::Color, coordinate::Coordinate};
use std::io::{self, stdout, Write};

pub fn human(board: &mut Board) {
    let mut x;
    let mut y;

    let color = board.turn;

    if board.is_pass(board.turn) {
        println!(
            "{}> passed",
            match color {
                Color::Black => "Black",
                Color::White => "White",
            }
        );
        board.turn_over();
        return;
    }

    loop {
        print!(
            "{} (x y): ",
            match color {
                Color::Black => "●",
                Color::White => "○",
            }
        );
        stdout().flush().ok();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let inputs: Vec<&str> = input.split_whitespace().collect();
                if inputs[0].chars().nth(0) == Some('p') {
                    println!(
                        "{}> passed",
                        match color {
                            Color::Black => "Black",
                            Color::White => "White",
                        }
                    );
                    board.turn_over();
                    return;
                }
                if inputs.len() < 2 {
                    continue;
                }
                match inputs[0].parse::<i32>() {
                    Ok(s) => x = s - 1,
                    Err(_) => continue,
                }
                match inputs[1].parse::<i32>() {
                    Ok(s) => y = s - 1,
                    Err(_) => continue,
                }

                if board.can_put(
                    match Coordinate::new(x, y) {
                        Ok(c) => c,
                        Err(_) => continue,
                    },
                    color,
                ) {
                    break;
                }
            }
            Err(_) => {
                eprintln!("Error occured while reading input.");
                continue;
            }
        };
    }
    println!(
        "{}> {} {}",
        match color {
            Color::Black => "●",
            Color::White => "○",
        },
        x + 1,
        y + 1
    );
    board.put(Coordinate::new(x, y).unwrap(), color);
    board.turn_over();
}
