use crate::prelude::*;

pub struct TGame {
    pub board: TBoard,
    pub random: usize,
    pub duration: i32,
    pub moves: usize,
    pub counter: usize,
    pub eog_flag: bool,
    pub status: String,
}

impl TGame {
    pub fn new_game() -> Self {
        qrand::srand(miniquad::date::now() as _);
        let random = qrand::gen_range::<usize>(0, PUZZLES);

        Self {
            board: TBoard::new_shuffled_board(),
            random,
            duration: 0,
            moves: 0,
            counter: 0,
            eog_flag: false,
            status: "".to_string(),
        }
    }
    pub fn play(&mut self, tile: usize) {
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
                if self.board.pieces[tile - 1].number == empty_tile {
                    // Move Left
                    self.board.pieces[tile - 1].number = self.board.pieces[tile].number;
                    self.board.pieces[tile].number = empty_tile;
                    self.moves += 1;
                }
            }
            if !last_column {
                if self.board.pieces[tile + 1].number == empty_tile {
                    // Move Right
                    self.board.pieces[tile + 1].number = self.board.pieces[tile].number;
                    self.board.pieces[tile].number = empty_tile;
                    self.moves += 1;
                }
            }
            if !first_row {
                if self.board.pieces[tile - COLUMNS].number == empty_tile {
                    // Move Up
                    self.board.pieces[tile - COLUMNS].number = self.board.pieces[tile].number;
                    self.board.pieces[tile].number = empty_tile;
                    self.moves += 1;
                }
            }
            if !last_row {
                if self.board.pieces[tile + COLUMNS].number == empty_tile {
                    // Move Down
                    self.board.pieces[tile + COLUMNS].number = self.board.pieces[tile].number;
                    self.board.pieces[tile].number = empty_tile;
                    self.moves += 1;
                }
            }
        }
    }
    pub fn win_check(&mut self) {
        self.counter = 0;
        for i in 0..COLUMNS * ROWS {
            if self.board.pieces[i].number == i {
                self.counter += 1;
            }
        }
        if self.counter == COLUMNS * ROWS {
            self.eog_flag = true;
            self.status = "Winner !!!".to_string();
        }
    }
}
