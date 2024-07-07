use crate::pieces::color::Color;
use crate::pieces::piece::Piece;

pub struct Bishop {
    color: Color,
    name: String,
}

impl Bishop {
    const NAME: char = 'B';
    pub fn new(color: Color) -> Bishop {
        let name = &color.get_color();
        Bishop {
            color,
            name: format!("{color}{name}", color = name, name = Bishop::NAME),
        }
    }
    pub fn can_move(&self, current_row: u8, current_column: u8, to_row: u8, to_column: u8) -> bool {
        if !self.is_valid_movement(&to_row, &to_column) {
            return false;
        }
        if current_row == to_row && current_column == to_column {
            return false;
        }
        let new_row = ((current_row as i8) - (to_row as i8)).abs();
        let new_column = ((to_column as i8) - (current_column as i8)).abs();
        new_column == new_row
    }
}
impl Piece for Bishop {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_movements::D;

    fn loop_through(row_manipulate: i32, column_manipulate: i32) {
        let mut current_row: i32 = 4;
        let mut current_column: i32 = D.into();

        let bishop = Bishop::new(Color::White);

        let add_or_sub_row = 1 * row_manipulate;
        let add_or_sub_column = 1 * column_manipulate;
        loop {
            // top left
            if current_row + add_or_sub_row <= 0
                || current_column + add_or_sub_column <= 0
                || current_row >= 7
                || current_column >= 7
            {
                break;
            }
            current_row += add_or_sub_row;
            current_column += add_or_sub_column;
            assert_eq!(
                bishop.can_move(
                    4,
                    D,
                    current_row.try_into().unwrap(),
                    current_column.try_into().unwrap()
                ),
                true
            );
        }
    }

    #[test]
    fn valid_bishop_movement() {
        let current_row: i32 = 4;
        let current_column: i32 = D.into();
        let bishop = Bishop::new(Color::White);
        assert_eq!(
            bishop.can_move(
                current_row.try_into().unwrap(),
                current_column.try_into().unwrap(),
                current_row.try_into().unwrap(),
                current_column.try_into().unwrap()
            ),
            false
        );
        // top left
        loop_through(-1, -1);
        // top right
        loop_through(-1, 1);
        // bottom left
        loop_through(1, -1);
        // bottom right
        loop_through(1, 1);
    }
}
