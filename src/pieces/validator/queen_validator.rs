use crate::pieces::validator::base_validator::BaseValidator;
use crate::pieces::validator::bishop_validator::BishopValidator;
use crate::pieces::validator::rook_validator::RookValidator;
use crate::pieces::validator::row_column::RowColumn;

pub struct QueenValidator<T: BaseValidator> {
    location_info: T,
}

impl<T: BaseValidator> QueenValidator<T> {
    pub fn new(location_info: T) -> QueenValidator<T> {
        QueenValidator { location_info }
    }
}

impl<T> BaseValidator for QueenValidator<T>
where
    T: Clone + BaseValidator,
{
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
        RookValidator::new(self.location_info.clone()).validate()
            || BishopValidator::new(self.location_info.clone()).validate()
    }
}
