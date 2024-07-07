use crate::game::game::History;
use crate::pieces::color::Color;
use crate::pieces::piece::PieceType;
use crate::pieces::validator::base_validator::BaseValidator;
use crate::pieces::validator::row_column::RowColumn;

pub struct PawnValidator<'a, T: BaseValidator> {
    location_info: T,
    to_has_piece: bool,
    color: &'a Color,
    moved: bool,
    last_history: Option<&'a History>,
}

impl<'a, T: BaseValidator> PawnValidator<'a, T> {
    pub fn new(
        location_info: T,
        to_has_piece: bool,
        color: &'a Color,
        moved: bool,
        history: Option<&'a History>,
    ) -> PawnValidator<'a, T> {
        PawnValidator {
            location_info,
            to_has_piece,
            color,
            moved,
            last_history: history,
        }
    }
}

impl<'a, T: BaseValidator> BaseValidator for PawnValidator<'a, T> {
    fn get_from(&self) -> &RowColumn {
        &self.location_info.get_from()
    }

    fn get_to(&self) -> &RowColumn {
        &self.location_info.get_to()
    }

    fn validate(&self) -> bool {
        if !self.location_info.validate() {
            return false;
        }

        let current_row = self.get_from().row;
        let current_column = self.get_from().column;
        let to_row = self.get_to().row;
        let to_column = self.get_to().column;
        if self.to_has_piece {
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

        if !self.is_valid_row_diff() {
            return false;
        }
        let new_row = ((current_row as i8) - (to_row as i8)).abs();
        let is_valid = new_row <= movement && current_column == to_column;
        is_valid
    }
}

impl<'a, T: BaseValidator> PawnValidator<'a, T> {
    fn is_valid_row_diff(&self) -> bool {
        let current_row = self.get_from().row;
        let current_column = self.get_from().column;
        let to_row = self.get_to().row;
        let distance = (current_row as i8) - (to_row as i8);
        if matches!(self.color, Color::White) && distance < 0 {
            return false;
        } else if matches!(self.color, Color::Black) && distance > 0 {
            return false;
        }
        return true;
    }
    pub fn is_en_passant(&self) -> bool {
        if !self.is_valid_row_diff() {
            return false;
        }
        if let Some(last_history) = self.last_history {
            let history_color = last_history.get_color();
            let color_diff;
            if matches!(self.color, Color::White) {
                color_diff = matches!(history_color, Color::Black)
            } else {
                color_diff = matches!(history_color, Color::White)
            }
            if matches!(last_history.get_piece_type(), PieceType::Pawn) && color_diff {
                let col_diff = (last_history.get_from().column as i8
                    - last_history.get_to().column as i8)
                    .abs();
                if col_diff != 0i8 {
                    return false;
                }
                let is_neighbor =
                    (self.get_from().column as i8 - last_history.get_to().column as i8).abs();
                if is_neighbor != 1 {
                    return false;
                }
                let row_diff = (self.get_from().row as i8 - self.get_to().row as i8).abs();
                return row_diff == 1 && last_history.get_to().column == self.get_to().column;
            }
        }
        false
    }
}
