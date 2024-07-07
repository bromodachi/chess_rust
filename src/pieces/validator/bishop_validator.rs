use crate::pieces::validator::base_validator::BaseValidator;
use crate::pieces::validator::row_column::RowColumn;

pub struct BishopValidator<T: BaseValidator> {
    location_info: T,
}

impl<T: BaseValidator> BishopValidator<T> {
    pub fn new(location_info: T) -> BishopValidator<T> {
        BishopValidator { location_info }
    }
}

impl<T: BaseValidator> BaseValidator for BishopValidator<T> {
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
        let new_row = ((from_row as i8) - (to_row as i8)).abs();
        let new_column = ((from_column as i8) - (to_column as i8)).abs();
        new_column == new_row
    }
}
