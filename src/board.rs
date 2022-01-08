use crate::prelude::*;

pub const BOARD_SIZE: usize = COLUMNS * ROWS;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TPiece {
    pub number: usize,
    pub selected: bool,
}

impl TPiece {
    pub fn new_piece(number: usize, selected: bool) -> Self {
        Self { number, selected }
    }
}

pub struct TBoard {
    pub pieces: Vec<TPiece>,
}
impl TBoard {
    pub fn new_sorted_board() -> Self {
        let mut board = TBoard { pieces: Vec::new() };
        for number in 0..BOARD_SIZE {
            board.pieces.push(TPiece::new_piece(number, false));
        }
        board
    }
    pub fn scramble(&mut self) {
        qrand::srand(miniquad::date::now() as _);

        for _ in 0..1024 {
            let tile = qrand::gen_range::<usize>(0, self.pieces.len());
            if tile < COLUMNS * ROWS {
                let empty_tile = COLUMNS * ROWS - 1;
                let mut first_column = false;
                let mut last_column = false;
                let mut first_row = false;
                let mut last_row = false;

                if tile % COLUMNS == 0 {
                    first_column = true;
                }
                if tile % COLUMNS == COLUMNS - 1 {
                    last_column = true;
                }
                if tile / COLUMNS == 0 {
                    first_row = true;
                }
                if tile / COLUMNS == ROWS - 1 {
                    last_row = true;
                }

                if !first_column {
                    if self.pieces[tile - 1].number == empty_tile {
                        // Move Left
                        self.pieces[tile - 1].number = self.pieces[tile].number;
                        self.pieces[tile].number = empty_tile;
                    }
                }
                if !last_column {
                    if self.pieces[tile + 1].number == empty_tile {
                        // Move Right
                        self.pieces[tile + 1].number = self.pieces[tile].number;
                        self.pieces[tile].number = empty_tile;
                    }
                }
                if !first_row {
                    if self.pieces[tile - COLUMNS].number == empty_tile {
                        // Move Up
                        self.pieces[tile - COLUMNS].number = self.pieces[tile].number;
                        self.pieces[tile].number = empty_tile;
                    }
                }
                if !last_row {
                    if self.pieces[tile + COLUMNS].number == empty_tile {
                        // Move Down
                        self.pieces[tile + COLUMNS].number = self.pieces[tile].number;
                        self.pieces[tile].number = empty_tile;
                    }
                }
            }
        }
    }
    pub fn new_shuffled_board() -> Self {
        let mut board = TBoard::new_sorted_board();
        TBoard::scramble(&mut board);
        board
    }
}
