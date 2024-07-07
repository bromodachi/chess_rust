use crate::pieces::validator::base_validator::BaseValidator;
use crate::pieces::validator::row_column::RowColumn;

pub struct RookValidator<T: BaseValidator> {
    location_info: T,
}

impl<T: BaseValidator> RookValidator<T> {
    pub fn new(location_info: T) -> RookValidator<T> {
        RookValidator { location_info }
    }
}

impl<T: BaseValidator> BaseValidator for RookValidator<T> {
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
        let from_row = self.location_info.get_from().row;
        let from_column = self.location_info.get_from().column;

        let to_row = self.location_info.get_to().row;
        let to_column = self.location_info.get_to().column;
        from_row == to_row || from_column == to_column
    }
}
