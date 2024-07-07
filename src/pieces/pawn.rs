use crate::pieces::color::Color;
use crate::pieces::piece::Piece;

pub struct Pawn {
    moved: bool,
    color: Color,
    name: String,
}

impl Pawn {
    const NAME: char = 'P';
    pub fn new(color: Color) -> Pawn {
        let name = &color.get_color();
        return Pawn {
            moved: false,
            color: color,
            name: format!("{color}{name}", color = name, name = Pawn::NAME),
        };
    }
    pub fn can_move(
        &self,
        current_row: u8,
        current_column: u8,
        to_row: u8,
        to_column: u8,
        is_attacking: bool,
    ) -> bool {
        if !self.is_valid_movement(&to_row, &to_column) {
            return false;
        }
        if is_attacking {
            let is_column_movement_valid =
                current_column as i8 - 1 == to_column as i8 || current_column + 1 == to_column;
            return (matches!(self.color, Color::White)
                && current_row - 1 == to_row
                && (is_column_movement_valid))
                || (current_row + 1 == to_row && (is_column_movement_valid));
        }
        // can't be the same.
        let mut movement = 1i8;
        if !self.moved {
            movement += 1;
        }

        let distance = (current_row as i8) - (to_row as i8);
        if matches!(self.color, Color::White) && distance < 0 {
            return false;
        } else if matches!(self.color, Color::Black) && distance > 0 {
            return false;
        }
        let new_row = ((current_row as i8) - (to_row as i8)).abs();
        return new_row <= movement && current_column == to_column;
    }
}

impl Piece for Pawn {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Pawn {
        fn test_new(color: Color, has_moved: bool) -> Pawn {
            Pawn {
                color,
                moved: has_moved,
                // don't care
                name: String::from("P"),
            }
        }
    }
    #[test]
    fn trying_to_move_out_of_bounds() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(6, 0, 8, 8, false), false)
    }

    #[test]
    fn first_move_should_be_able_to_move_up_two_spaces() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(6, 0, 4, 0, false), true);
    }

    #[test]
    fn first_move_should_be_able_to_move_up_one_space() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(6, 0, 5, 0, false), true);
    }

    #[test]
    fn first_move_shouldnt_be_able_to_move_three_spaces() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(6, 0, 3, 0, false), false);
    }

    #[test]
    fn already_moved_shouldnt_be_able_to_move_two_spaces() {
        let pawn = Pawn::test_new(Color::White, true);
        assert_eq!(pawn.can_move(6, 0, 4, 0, false), false);
    }

    #[test]
    fn is_attacking_only_diagonal() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(7, 0, 6, 0, true), false);
    }

    #[test]
    fn is_attacking_only_diagonal_valid() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(7, 0, 6, 1, true), true);
    }

    #[test]
    fn is_attacking_only_diagonal_but_too_far() {
        let pawn = Pawn::new(Color::White);
        assert_eq!(pawn.can_move(7, 0, 6, 2, true), false);
    }
}
