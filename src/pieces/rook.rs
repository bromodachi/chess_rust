use crate::pieces::color::Color;
use crate::pieces::piece::Piece;

pub struct Rook {
    color: Color,
    name: String,
}
impl Rook {
    const NAME: char = 'R';
    pub fn new(color: Color) -> Rook {
        let name = &color.get_color();
        return Rook {
            color,
            name: format!("{color}{name}", color = name.clone(), name = Rook::NAME),
        };
    }

    // TODO: handle if other piece exists.
    pub fn can_move(&self, current_row: u8, current_column: u8, to_row: u8, to_column: u8) -> bool {
        // can't be the same.
        if !self.is_valid_movement(&to_row, &to_column) {
            return false;
        }
        if current_row == to_row && current_column == to_column {
            return false;
        }
        current_row == to_row || current_column == to_column
    }
}

impl Piece for Rook {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_movements::*;

    #[test]
    fn trying_to_move_out_of_bounds() {
        let current_rook = Rook::new(Color::White);
        assert_eq!(current_rook.can_move(7, 0, 8, 7), false);
    }

    #[test]
    fn trying_to_move_to_current_spot() {
        let current_rook = Rook::new(Color::White);
        assert_eq!(current_rook.can_move(7, 0, 7, 0), false);
    }

    #[test]
    fn trying_to_move_to_diagonal() {
        let current_rook = Rook::new(Color::White);
        assert_eq!(current_rook.can_move(7, 0, 6, 1), false);
    }

    #[test]
    fn trying_to_move_to_upwards() {
        let current_rook = Rook::new(Color::White);
        assert_eq!(current_rook.can_move(7, 0, 0, 0), true);
    }

    #[test]
    fn trying_to_move_to_right() {
        let current_rook = Rook::new(Color::White);
        assert_eq!(current_rook.can_move(7, 0, 7, 7), true);
    }

    #[test]
    fn invalid_rook_movements() {
        let curr_row = 3;
        let curr_column = D;
        let columns = [A, B, C, E, F, G, H];
        let rows: [u8; 7] = [0, 1, 2, 4, 5, 6, 7];

        for row in rows {
            for column in columns {
                let current_rook = Rook::new(Color::White);
                assert_eq!(
                    current_rook.can_move(curr_row, curr_column, row, column),
                    false
                );
            }
        }
    }
}
