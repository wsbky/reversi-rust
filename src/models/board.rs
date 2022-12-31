use super::{color::Color, coordinate::Coordinate};
use std::fmt;

pub enum Judge {
    BlackWins,
    WhiteWins,
    Draw,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub black: u64,
    pub white: u64,

    show_legalputs: bool,
    pub turn: Color,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            black: 0x0000000810000000,
            white: 0x0000001008000000,
            show_legalputs: false,
            turn: Color::Black,
        }
    }
}

impl Board {
    pub fn new_init() -> Self {
        Self {
            ..Default::default()
        }
    }
    #[allow(dead_code)]
    pub fn new(black: u64, white: u64) -> Self {
        Self {
            black,
            white,
            ..Default::default()
        }
    }

    pub fn from(other: &Board) -> Self {
        other.clone()
    }

    pub fn legal_mode(&self, turn: Color) -> Self {
        let mut r = Board::from(&self);
        r.show_legalputs = true;
        r.turn = turn;
        r
    }

    pub fn board_of(&mut self, color: Color) -> &mut u64 {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut r: String = String::new();
        r.push_str("   1 2 3 4 5 6 7 8  \n");
        r.push_str("  ----------------- \n");

        for i in 0..8 {
            r.push_str(format!("{}| ", i + 1).as_str());

            for j in 0..8 {
                let place = Coordinate::new(j, i).unwrap();
                if self.black & place.bit != 0 {
                    r.push('●');
                } else if self.white & place.bit != 0 {
                    r.push('○');
                } else if self.show_legalputs && self.can_put(place, self.turn) {
                    r.push('+')
                } else {
                    r.push('∙');
                }

                if j != 7 {
                    r.push(' ');
                }
            }

            r.push_str(" |\n");
        }
        r.push_str("  ----------------- ");

        write!(f, "\n{}\n", r)
    }
}

impl Board {
    pub fn put(&mut self, place: Coordinate, color: Color) -> Self {
        let mut my_board: u64 = self.board_of(color).clone();
        let mut op_board: u64 = self.board_of(color.other()).clone();

        let mut rev: u64 = 0;

        for i in 0..8 {
            let mut temp: u64 = 0;
            let mut mask: Coordinate = place.transfer(i);
            while mask.bit != 0 && (mask.bit & op_board) != 0 {
                temp |= mask.bit;
                mask = mask.transfer(i);
            }
            if (mask.bit & my_board) != 0 {
                rev |= temp;
            }
        }

        my_board ^= place.bit | rev;
        op_board ^= rev;

        *self.board_of(color) = my_board;
        *self.board_of(color.other()) = op_board;

        *self
    }
}

impl Board {
    fn popcount_for(self, color: Color) -> i32 {
        let mut x: u64 = *self.clone().board_of(color);

        x = x - ((x >> 1) & 0x5555555555555555);

        x = (x & 0x3333333333333333) + ((x >> 2) & 0x3333333333333333);

        x = (x + (x >> 4)) & 0x0f0f0f0f0f0f0f0f;
        x = x + (x >> 8);
        x = x + (x >> 16);
        x = x + (x >> 32);

        (x & 0x0000007f) as i32
    }

    pub fn legal_board(&self, color: Color) -> u64 {
        let my_board: u64 = *self.clone().board_of(color);
        let op_board: u64 = *self.clone().board_of(color.other());
        let blank_board: u64 = !(my_board | op_board);

        let hor_watch_op_board: u64 = op_board & 0x7e7e7e7e7e7e7e7e;
        let ver_watch_op_board: u64 = op_board & 0x00ffffffffffff00;
        let all_watch_op_board: u64 = op_board & 0x007e7e7e7e7e7e00;

        let mut legal_board: u64 = 0;
        let mut temp: u64;

        // left
        temp = hor_watch_op_board & (my_board >> 1);
        for _ in 0..5 {
            temp |= hor_watch_op_board & (temp >> 1);
        }
        legal_board |= blank_board & (temp >> 1);

        // right
        temp = hor_watch_op_board & (my_board << 1);
        for _ in 0..5 {
            temp |= hor_watch_op_board & (temp << 1);
        }
        legal_board |= blank_board & (temp << 1);

        // upper
        temp = ver_watch_op_board & (my_board >> 8);
        for _ in 0..5 {
            temp |= ver_watch_op_board & (temp >> 8);
        }
        legal_board |= blank_board & (temp >> 8);

        // lower
        temp = ver_watch_op_board & (my_board << 8);
        for _ in 0..5 {
            temp |= ver_watch_op_board & (temp << 8);
        }
        legal_board |= blank_board & (temp << 8);

        // upper left
        temp = all_watch_op_board & (my_board >> 9);
        for _ in 0..5 {
            temp |= all_watch_op_board & (temp >> 9);
        }
        legal_board |= blank_board & (temp >> 9);

        // upper right
        temp = all_watch_op_board & (my_board >> 7);
        for _ in 0..5 {
            temp |= all_watch_op_board & (temp >> 7);
        }
        legal_board |= blank_board & (temp >> 7);

        // lower left
        temp = all_watch_op_board & (my_board << 7);
        for _ in 0..5 {
            temp |= all_watch_op_board & (temp << 7);
        }
        legal_board |= blank_board & (temp << 7);

        // lower right
        temp = all_watch_op_board & (my_board << 9);
        for _ in 0..5 {
            temp |= all_watch_op_board & (temp << 9);
        }
        legal_board |= blank_board & (temp << 9);

        legal_board
    }

    pub fn can_put(&self, place: Coordinate, color: Color) -> bool {
        let bit = place.bit;
        (bit & self.legal_board(color)) == bit
    }

    pub fn is_pass(&self, color: Color) -> bool {
        self.legal_board(color) == 0 && self.legal_board(color.other()) != 0
    }

    pub fn is_over(&self) -> bool {
        self.legal_board(Color::Black) == 0 && self.legal_board(Color::White) == 0
    }

    pub fn turn_over(&mut self) {
        self.turn = self.turn.other();
    }

    pub fn judge(&self) -> Judge {
        let black_count = self.popcount_for(Color::Black);
        let white_count = self.popcount_for(Color::White);

        if black_count > white_count {
            Judge::BlackWins
        } else if black_count < white_count {
            Judge::WhiteWins
        } else {
            Judge::Draw
        }
    }
}
