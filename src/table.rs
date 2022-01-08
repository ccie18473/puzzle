use crate::prelude::*;

pub struct TTable {
    pub game: TGame,
    pub puzzle_width: f32,
    pub puzzle_height: f32,
    pub piece_width: f32,
    pub piece_height: f32,
}

impl TTable {
    pub fn new_table() -> Self {
        Self {
            game: TGame::new_game(),
            puzzle_width: 0.0,
            puzzle_height: 0.0,
            piece_width: 0.0,
            piece_height: 0.0,
        }
    }
    pub fn table_pos(&mut self, x: f32, y: f32) -> usize {
        if x < self.puzzle_width && y < self.puzzle_height {
            let x_int = (x / self.piece_width) as usize;
            let y_int = (y / self.piece_height) as usize;
            let tile = x_int + y_int * COLUMNS;
            tile as usize
        } else {
            99
        }
    }
    pub fn process_mouse_down(&mut self, x: f32, y: f32) {
        let tile = self.table_pos(x, y);
        if !self.game.eog_flag {
            self.game.play(tile);
        } else {
            self.game = TGame::new_game();
        }
    }
    pub fn process_mouse_up(&mut self, _x: f32, _y: f32) {
        //
    }
}
