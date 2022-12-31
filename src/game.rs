use crate::models::board::Board;

use self::player::human;

mod player;

pub fn game_main() {
    let mut board = Board::new_init();

    let mut is_over = false;
    while !is_over {
        println!("{}", board.legal_mode(board.turn));
        human(&mut board);
        is_over = board.is_over();
    }
    println!(
        "{}",
        match board.judge() {
            crate::models::board::Judge::BlackWins => "Black wins.",
            crate::models::board::Judge::WhiteWins => "White wins.",
            crate::models::board::Judge::Draw => "Draw.",
        }
    );
    println!("{}", board.legal_mode(board.turn));
}
