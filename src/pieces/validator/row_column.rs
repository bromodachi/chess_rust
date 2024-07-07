#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RowColumn {
    pub row: u8,
    pub column: u8,
}
impl RowColumn {
    pub fn new(row: u8, column: u8) -> RowColumn {
        RowColumn { row, column }
    }
}

#[derive(Clone)]
pub struct BaseLocation {
    pub from: RowColumn,
    pub to: RowColumn,
}

impl BaseLocation {
    pub fn new(from_row: u8, from_column: u8, to_row: u8, to_column: u8) -> BaseLocation {
        BaseLocation {
            from: RowColumn::new(from_row, from_column),
            to: RowColumn::new(to_row, to_column),
        }
    }

    pub fn new_row_column(from_row: RowColumn, to_row: RowColumn) -> BaseLocation {
        BaseLocation {
            from: from_row,
            to: to_row,
        }
    }

    fn invalid_movements(&self, value: &u8) -> bool {
        value < &0u8 || value > &7u8
    }
    pub fn is_valid_movement(&self, to_row: &u8, to_column: &u8) -> bool {
        !self.invalid_movements(to_row) && !self.invalid_movements(to_column)
    }
}
