use crate::game::board::Board;
use crate::pieces::validator::row_column::{BaseLocation, RowColumn};

pub trait BaseValidator {
    fn get_from(&self) -> &RowColumn;
    fn get_to(&self) -> &RowColumn;

    fn validate(&self) -> bool;
}

pub trait ContainsConflictingPiece {
    fn can_move(&self, board: &Board) -> bool;
}

impl<R: BaseValidator> ContainsConflictingPiece for R {
    fn can_move(&self, board: &Board) -> bool {
        let from = self.get_from();
        let to = self.get_to();
        let mut new_row = (to.row as i8) - (from.row as i8);
        let mut new_column = (to.column as i8) - (from.column as i8);
        let increment_row = if new_row == 0 {
            0
        } else {
            if new_row < 0 {
                -1
            } else {
                1
            }
        };
        let increment_column = if new_column == 0 {
            0
        } else {
            if new_column < 0 {
                -1
            } else {
                1
            }
        };
        if new_row == 0 && to.row == from.row {
            new_row = to.row as i8;
        }
        if new_column == 0 && to.column == from.column {
            new_column = to.column as i8;
        }
        let mut from_row = from.row as i8;
        let mut from_column = from.column as i8;
        loop {
            from_row += increment_row;
            from_column += increment_column;
            if from_row < 0 || from_column < 0 {
                return false;
            }
            if from_row == to.row as i8 && from_column == to.column as i8 {
                return true;
            }
            if board.squares[from_row as usize][from_column as usize].has_piece() {
                return false;
            }
        }
    }
}

impl BaseValidator for BaseLocation {
    fn get_from(&self) -> &RowColumn {
        return &self.from;
    }

    fn get_to(&self) -> &RowColumn {
        return &self.to;
    }

    fn validate(&self) -> bool {
        self.is_valid_movement(&self.get_to().row, &self.get_to().column) &&
            // can't be the same spot
            !(&self.get_from().row == &self.get_to().row && &self.get_from().column == &self.get_to().column)
    }
}

#[cfg(test)]
mod tests {
    use crate::board_movements::{C, D};
    use crate::pieces::validator::base_validator::BaseValidator;
    use crate::pieces::validator::bishop_validator::BishopValidator;
    use crate::pieces::validator::queen_validator::QueenValidator;
    use crate::pieces::validator::rook_validator::RookValidator;
    use crate::pieces::validator::row_column::BaseLocation;

    #[test]
    fn rook_validator() {
        let base_validator = BaseLocation::new(7, 0, 8, 7);
        let rook_validator = RookValidator::new(base_validator);
        assert_eq!(rook_validator.validate(), false);
    }

    #[test]
    fn trying_to_move_to_current_spot() {
        let base_validator = BaseLocation::new(7, 0, 7, 0);
        let rook_validator = RookValidator::new(base_validator);
        assert_eq!(rook_validator.validate(), false);
    }

    #[test]
    fn trying_to_move_to_diagonal() {
        let base_validator = BaseLocation::new(7, 0, 6, 1);
        let rook_validator = RookValidator::new(base_validator);
        assert_eq!(rook_validator.validate(), false);
    }

    #[test]
    fn trying_to_move_to_upwards() {
        let base_validator = BaseLocation::new(7, 0, 0, 0);
        let rook_validator = RookValidator::new(base_validator);
        assert_eq!(rook_validator.validate(), true);
    }

    #[test]
    fn trying_to_move_to_right() {
        let base_validator = BaseLocation::new(7, 0, 7, 7);
        let rook_validator = RookValidator::new(base_validator);
        assert_eq!(rook_validator.validate(), true);
    }

    fn loop_through(row_manipulate: i32, column_manipulate: i32) {
        let mut current_row: i32 = 4;
        let mut current_column: i32 = D.into();

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
            let base_validator = BaseLocation::new(
                4,
                D,
                current_row.try_into().unwrap(),
                current_column.try_into().unwrap(),
            );
            let bishop_validator = BishopValidator::new(base_validator);
            assert_eq!(bishop_validator.validate(), true);
        }
    }

    #[test]
    fn valid_bishop_movement() {
        let current_row: i32 = 4;
        let current_column: i32 = D.into();
        let base_validator = BaseLocation::new(
            current_row.try_into().unwrap(),
            current_column.try_into().unwrap(),
            current_row.try_into().unwrap(),
            current_column.try_into().unwrap(),
        );
        let bishop_validator = BishopValidator::new(base_validator);
        assert_eq!(bishop_validator.validate(), false);
        // top left
        loop_through(-1, -1);
        // top right
        loop_through(-1, 1);
        // bottom left
        loop_through(1, -1);
        // bottom right
        loop_through(1, 1);
    }

    fn get_queen_validator(
        from_row: u8,
        from_column: u8,
        to_row: u8,
        to_column: u8,
    ) -> QueenValidator<BaseLocation> {
        QueenValidator::new(BaseLocation::new(
            from_row.try_into().unwrap(),
            from_column.try_into().unwrap(),
            to_row.try_into().unwrap(),
            to_column.try_into().unwrap(),
        ))
    }

    #[test]
    fn valid_queen_movement() {
        assert_eq!(
            get_queen_validator(4, D.into(), 5, D.into()).validate(),
            true
        );

        assert_eq!(
            get_queen_validator(4, D.into(), 5, C.into()).validate(),
            true
        );
    }
}
