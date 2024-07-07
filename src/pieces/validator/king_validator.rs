use crate::board_movements::{A, D, F, H};
use crate::game::board::Board;
use crate::pieces::piece::PieceInfo;
use crate::pieces::validator::base_validator::{BaseValidator, ContainsConflictingPiece};
use crate::pieces::validator::row_column::{BaseLocation, RowColumn};

pub struct KingValidator<'a, T: BaseValidator> {
    location_info: T,
    king_info: &'a PieceInfo,
}

impl<'a, T: BaseValidator> KingValidator<'a, T> {
    pub fn new(location_info: T, king_info: &PieceInfo) -> KingValidator<T> {
        KingValidator {
            location_info,
            king_info,
        }
    }
}

impl<'a, T: BaseValidator> KingValidator<'a, T> {
    ///
    ///
    /// # Arguments
    ///
    /// * `board`:
    ///
    /// returns: <unknown>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn is_castling(&self, board: &Board) -> Option<BaseLocation> {
        if self.king_info.get_has_moved() {
            return None;
        }
        let from = self.get_from();
        let to = self.get_to();
        let from_row: i8 = from.row as i8;
        let to_row: i8 = to.row as i8;
        // not castling. It needs to move left or right two spaces
        if (from_row - to_row).abs() != 0 {
            return None;
        }
        let from_column: i8 = from.column as i8;
        let to_column: i8 = to.column as i8;
        // once again, needs to be two spaces to the left or right;
        if (from_column - to_column).abs() != 2 {
            return None;
        }
        // if it can move to that space(no pieces blocking)

        if self.can_move(&board) {
            let is_left = (to_column - from_column) < 0;
            let from_column = if is_left { A } else { H };
            let to_column = if is_left { D } else { F };
            Some(BaseLocation::new_row_column(
                RowColumn::new(from.row, from_column),
                RowColumn::new(to.row, to_column),
            ))
        } else {
            None
        }
    }
}

impl<'a, T: BaseValidator> BaseValidator for KingValidator<'a, T> {
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
        let new_row = (current_row as i8 - to_row as i8).abs();
        let new_column = (current_column as i8 - to_column as i8).abs();

        new_row <= 1 && new_column <= 1
    }
}
