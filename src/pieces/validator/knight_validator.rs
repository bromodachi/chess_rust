use crate::pieces::validator::base_validator::BaseValidator;
use crate::pieces::validator::row_column::RowColumn;

pub struct KnightValidator<T: BaseValidator> {
    location_info: T,
}

impl<T: BaseValidator> KnightValidator<T> {
    pub fn new(location_info: T) -> KnightValidator<T> {
        KnightValidator { location_info }
    }
}

impl<T: BaseValidator> BaseValidator for KnightValidator<T> {
    fn get_from(&self) -> &RowColumn {
        &self.location_info.get_from()
    }

    fn get_to(&self) -> &RowColumn {
        &self.location_info.get_to()
    }

    // TODO: Test this
    fn validate(&self) -> bool {
        if !self.location_info.validate() {
            return false;
        }
        let current_row = self.get_from().row;
        let current_column = self.get_from().column;
        let to_row = self.get_to().row;
        let to_column = self.get_to().column;
        let new_row = ((current_row as i8) - (to_row as i8)).abs();
        let new_column = ((current_column as i8) - (to_column as i8)).abs();
        if new_row == 0 && new_column == 0 {
            return false;
        }
        return new_row == 1 && new_column == 2 || new_row == 2 && new_column == 1;
    }
}
