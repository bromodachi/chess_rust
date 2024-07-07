use crate::pieces::color::Color;
use crate::pieces::piece::Piece;

pub struct Knight {
    color: Color,
    name: String,
}

impl Knight {
    const NAME: char = 'N';
    pub fn new(color: Color) -> Knight {
        let name = &color.get_color();
        Knight {
            color,
            name: format!("{color}{name}", color = name, name = Knight::NAME),
        }
    }
    pub fn can_move(&self, current_row: u8, current_column: u8, to_row: u8, to_column: u8) -> bool {
        if !self.is_valid_movement(&to_row, &to_column) {
            return false;
        }
        let new_row = (current_row as i8 - to_row as i8).abs();
        let new_column = (current_column as i8 - to_column as i8).abs();

        return new_row == 1 && new_column == 2 || new_column == 1 && new_row == 2;
    }
}

impl Piece for Knight {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_movements::{B, C, D, E, F};
    use crate::utils::pair::Pair;

    #[test]
    fn valid_movements() {
        let start_row = 3;
        let start_column = D;

        // left will be our row
        // right will be our column
        let knight = Knight::new(Color::White);

        let pairs: [Pair<i32, i32>; 8] = [
            // top left
            Pair::new(1i32, C as i32),
            Pair::new(2i32, B as i32),
            // top right
            Pair::new(1i32, E as i32),
            Pair::new(2i32, F as i32),
            // bottom left
            Pair::new(4i32, B as i32),
            Pair::new(5i32, C as i32),
            // bottom left
            Pair::new(5i32, E as i32),
            Pair::new(4i32, F as i32),
        ];
        for p in &pairs {
            assert_eq!(
                knight.can_move(
                    start_row,
                    start_column,
                    p.left.try_into().unwrap(),
                    p.right.try_into().unwrap()
                ),
                true
            );
        }
    }
    #[test]
    fn invalid_movements() {
        let pairs: [Pair<i32, i32>; 8] = [
            // top left
            Pair::new(1i32, C as i32),
            Pair::new(2i32, B as i32),
            // top right
            Pair::new(1i32, E as i32),
            Pair::new(2i32, F as i32),
            // bottom left
            Pair::new(4i32, B as i32),
            Pair::new(5i32, C as i32),
            // bottom left
            Pair::new(5i32, E as i32),
            Pair::new(4i32, F as i32),
        ];

        let start_row = 3;
        let start_column = D;

        let knight = Knight::new(Color::White);

        for row in 0..8 {
            for column in 0..8 {
                if pairs.contains(&Pair {
                    left: row,
                    right: column,
                }) {
                    continue;
                }
                assert_eq!(
                    knight.can_move(
                        start_row,
                        start_column,
                        row.try_into().unwrap(),
                        column.try_into().unwrap()
                    ),
                    false
                );
            }
        }
    }
}
