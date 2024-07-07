// TODO: Just a combination of everything. Since we solved all the other pieces, it should be
// easy to implement

use crate::pieces::bishop::Bishop;
use crate::pieces::color::Color;
use crate::pieces::piece::Piece;
use crate::pieces::rook::Rook;

pub struct Queen {
    color: Color,
    name: String,
}

impl Queen {
    const NAME: char = 'Q';
    pub fn new(color: Color) -> Queen {
        let name = &color.get_color();
        Queen {
            color,
            name: format!("{color}{name}", color = name, name = Queen::NAME),
        }
    }
    pub fn can_move(&self, current_row: u8, current_column: u8, to_row: u8, to_column: u8) -> bool {
        if !self.is_valid_movement(&to_row, &to_column) {
            return false;
        }

        Bishop::new(self.color.clone()).can_move(current_row, current_column, to_row, to_column)
            || Rook::new(self.color.clone()).can_move(
                current_row,
                current_column,
                to_row,
                to_column,
            )
    }
}

impl Piece for Queen {
    fn get_name(&self) -> &str {
        &self.name
    }
}
