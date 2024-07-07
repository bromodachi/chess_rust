use crate::pieces::color::Color;
use crate::pieces::piece::Piece;

pub struct King {
    color: Color,
    name: String,
}

impl Piece for King {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl King {
    const NAME: char = 'K';
    pub fn new(color: Color) -> King {
        let name = &color.get_color();
        King {
            color,
            name: format!("{color}{name}", color = name, name = King::NAME),
        }
    }
    // TODO: We want to take references instead.
    pub fn can_move(&self, current_row: u8, current_column: u8, to_row: u8, to_column: u8) -> bool {
        if !self.is_valid_movement(&to_row, &to_column) {
            return false;
        }
        let new_row = ((current_row as i8) - (to_row as i8)).abs();
        let new_column = ((current_column as i8) - (to_column as i8)).abs();
        if new_row == 0 && new_column == 0 {
            return false;
        }
        new_row <= 1 && new_column <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_movements::*;

    #[test]
    fn king_in_middle_all_valid_movements() {
        let king = King::new(Color::White);
        let king_current_row_placement = 5;
        let king_current_column_placement = F;

        let valid_columns = [E, F, G];
        let valid_rows = [6, 5, 4];

        for row in &valid_rows {
            for column in &valid_columns {
                let equality = !(*column == F && *row == 5);
                assert_eq!(
                    king.can_move(
                        king_current_row_placement,
                        king_current_column_placement,
                        *row,
                        *column
                    ),
                    equality
                )
            }
        }
    }

    #[test]
    fn king_invalid_movements() {
        let king = King::new(Color::White);
        let king_current_row_placement = 5;
        let king_current_column_placement = F;

        let valid_columns = [E, F, G];
        let valid_rows = [8, 7, 3, 2, 1];

        for row in &valid_rows {
            for column in &valid_columns {
                assert_eq!(
                    king.can_move(
                        king_current_row_placement,
                        king_current_column_placement,
                        *row,
                        *column
                    ),
                    false
                )
            }
        }
    }
}
