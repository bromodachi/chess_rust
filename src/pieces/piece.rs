use crate::game::board::Board;
use crate::game::game::History;
use crate::pieces::color::Color;
use crate::pieces::piece::PieceInfo::{Default, MovementInfo};
use crate::pieces::validator::base_validator::{BaseValidator, ContainsConflictingPiece};
use crate::pieces::validator::bishop_validator::BishopValidator;
use crate::pieces::validator::king_validator::KingValidator;
use crate::pieces::validator::knight_validator::KnightValidator;
use crate::pieces::validator::pawn_validator::PawnValidator;
use crate::pieces::validator::queen_validator::QueenValidator;
use crate::pieces::validator::rook_validator::RookValidator;
use crate::pieces::validator::row_column::{BaseLocation, RowColumn};

pub trait Piece {
    fn get_name(&self) -> &str;
    fn invalid_movements(&self, value: &u8) -> bool {
        value < &0u8 || value > &7u8
    }
    fn is_valid_movement(&self, to_row: &u8, to_column: &u8) -> bool {
        !self.invalid_movements(to_row) && !self.invalid_movements(to_column)
    }
}

#[derive(Clone)]
pub enum PieceInfo {
    Default(PieceProperty),
    MovementInfo(PieceMovedProperty),
}
impl PieceInfo {
    // TODO: Always track movement.
    // Might just change this to a struct instead
    pub fn new(color: Color, track_movement: bool) -> PieceInfo {
        if track_movement {
            MovementInfo(PieceMovedProperty {
                color: color,
                has_moved: false,
            })
        } else {
            Default(PieceProperty { color })
        }
    }
    fn get_color_name(&self) -> String {
        match self {
            PieceInfo::Default(piece_prop) => piece_prop.get_color_name(),
            PieceInfo::MovementInfo(pawn_prop) => pawn_prop.get_color_name(),
        }
    }

    pub fn set_has_moved(&mut self) {
        match self {
            PieceInfo::Default(_default) => {
                // does nothing
            }
            PieceInfo::MovementInfo(pawn_prop) => pawn_prop.has_moved = true,
        }
    }

    pub fn get_has_moved(&self) -> bool {
        match self {
            Default(_) => return false,
            MovementInfo(pawn_prop) => return pawn_prop.has_moved,
        }
    }

    fn get_color(&self) -> &Color {
        match self {
            Default(prop) => &prop.color,
            MovementInfo(prop) => &prop.color,
        }
    }
}

trait PiecePropertyDefault<Color> {
    fn get_color_name(&self) -> String;
    fn get_color(&self) -> &Color;
}

#[derive(Clone)]
pub struct PieceProperty {
    color: Color,
}

#[derive(Clone)]
pub struct PieceMovedProperty {
    color: Color,
    has_moved: bool,
}

impl PiecePropertyDefault<Color> for PieceProperty {
    fn get_color_name(&self) -> String {
        return self.color.get_color();
    }

    fn get_color(&self) -> &Color {
        return &self.color;
    }
}

impl PiecePropertyDefault<Color> for PieceMovedProperty {
    fn get_color_name(&self) -> String {
        return self.color.get_color();
    }

    fn get_color(&self) -> &Color {
        return &self.color;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    King,
    Queen,
    Rook,
}

impl PieceType {
    pub fn create_actual_piece(&self, piece_info: PieceInfo) -> Pieces {
        match self {
            PieceType::Pawn => Pieces::Pawn(piece_info),
            PieceType::Bishop => Pieces::Bishop(piece_info),
            PieceType::Knight => Pieces::Knight(piece_info),
            PieceType::King => Pieces::King(piece_info),
            PieceType::Queen => Pieces::Queen(piece_info),
            PieceType::Rook => Pieces::Rook(piece_info),
        }
    }
}

#[derive(Clone)]
pub enum Pieces {
    Pawn(PieceInfo),
    Bishop(PieceInfo),
    Knight(PieceInfo),
    King(PieceInfo),
    Queen(PieceInfo),
    Rook(PieceInfo),
}

pub enum ValidMovement {
    INVALID,
    VALID,
    CASTLING(BaseLocation),
    EnPassant(RowColumn),
    Promotion,
}

impl Pieces {
    pub fn set_as_moved(&mut self) {
        match self {
            Pieces::Pawn(ref mut info) => {
                info.set_has_moved();
            }
            Pieces::King(ref mut info) => {
                info.set_has_moved();
            }
            Pieces::Rook(ref mut info) => {
                info.set_has_moved();
            }
            _ => {}
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            Pieces::Pawn(piece_info) => format!("{}P", &piece_info.get_color_name()),
            Pieces::Bishop(piece_info) => format!("{}B", &piece_info.get_color_name()),
            Pieces::King(piece_info) => format!("{}K", &piece_info.get_color_name()),
            Pieces::Queen(piece_info) => format!("{}Q", &piece_info.get_color_name()),
            Pieces::Rook(piece_info) => format!("{}R", &piece_info.get_color_name()),
            Pieces::Knight(piece_info) => format!("{}N", &piece_info.get_color_name()),
        }
    }

    pub fn get_color(&self) -> &Color {
        match self {
            Pieces::Pawn(piece_info) => piece_info.get_color(),
            Pieces::Bishop(piece_info) => piece_info.get_color(),
            Pieces::King(piece_info) => piece_info.get_color(),
            Pieces::Queen(piece_info) => piece_info.get_color(),
            Pieces::Rook(piece_info) => piece_info.get_color(),
            Pieces::Knight(piece_info) => piece_info.get_color(),
        }
    }

    pub fn get_piece_type(&self) -> PieceType {
        match self {
            Pieces::Pawn(_) => PieceType::Pawn,
            Pieces::Bishop(_) => PieceType::Bishop,
            Pieces::Knight(_) => PieceType::Knight,
            Pieces::King(_) => PieceType::King,
            Pieces::Queen(_) => PieceType::Queen,
            Pieces::Rook(_) => PieceType::Rook,
        }
    }

    pub fn get_piece_info(&self) -> &PieceInfo {
        match self {
            Pieces::Pawn(info) => info,
            Pieces::Bishop(info) => info,
            Pieces::Knight(info) => info,
            Pieces::King(info) => info,
            Pieces::Queen(info) => info,
            Pieces::Rook(info) => info,
        }
    }

    pub fn get_piece_info_clone(&self) -> PieceInfo {
        match self {
            Pieces::Pawn(info) => info.clone(),
            Pieces::Bishop(info) => info.clone(),
            Pieces::Knight(info) => info.clone(),
            Pieces::King(info) => info.clone(),
            Pieces::Queen(info) => info.clone(),
            Pieces::Rook(info) => info.clone(),
        }
    }

    fn get_increment(&self, value: i8) -> i8 {
        if value == 0 {
            0
        } else if value > 0 {
            1
        } else {
            -1
        }
    }
    pub fn get_movements_between(&self, from: &RowColumn, to: &RowColumn) -> Vec<RowColumn> {
        let mut from_row = from.row as i8;
        let mut from_column = from.column as i8;
        let new_row = (to.row as i8) - (from.row as i8);
        let new_column = (to.column as i8) - (from.column as i8);
        let row_increment = self.get_increment(new_row);
        let column_increment = self.get_increment(new_column);
        let mut row_columns = vec![];
        loop {
            from_row += row_increment;
            from_column += column_increment;
            if from_row == to.row as i8 && from_column == to.column as i8 {
                break;
            }
            row_columns.push(RowColumn::new(from_row as u8, from_column as u8));
        }
        row_columns
    }

    pub fn is_valid_movement(
        &self,
        from: &RowColumn,
        to: &RowColumn,
        board: &Board,
        last_history: Option<&History>,
    ) -> ValidMovement {
        let base_location = BaseLocation::new_row_column(from.clone(), to.clone());

        let is_valid: bool = match self {
            Pieces::Pawn(pawn) => {
                let validator = PawnValidator::new(
                    base_location,
                    board.squares[to.row as usize][to.column as usize].has_piece(),
                    &pawn.get_color(),
                    pawn.get_has_moved(),
                    last_history,
                );
                if validator.validate() {
                    if to.row == 7 || to.row == 0 {
                        return ValidMovement::Promotion;
                    }
                    true
                } else {
                    if validator.is_en_passant() {
                        return ValidMovement::EnPassant(last_history.unwrap().get_to().clone());
                    }
                    false
                }
            }
            Pieces::Bishop(_bishop) => {
                let validator = BishopValidator::new(base_location);
                validator.validate() && validator.can_move(&board)
            }
            Pieces::King(king_info) => {
                let validator = KingValidator::new(base_location, &king_info);
                if let Some((Some(piece_type), position_of_rook, Some(piece_info))) =
                    validator.is_castling(&board).and_then(|rook_pos| {
                        let from_row = rook_pos.from.row;
                        let from_column = rook_pos.from.column;
                        Some((
                            board.squares[from_row as usize][from_column as usize].get_piece_type(),
                            rook_pos,
                            board.squares[from_row as usize][from_column as usize].get_piece_info(),
                        ))
                    })
                {
                    if matches!(piece_type, PieceType::Rook) {
                        let validator = RookValidator::new(position_of_rook.clone());
                        if !piece_info.get_has_moved() && validator.can_move(&board) {
                            return ValidMovement::CASTLING(position_of_rook.clone());
                        }
                    }
                }
                validator.validate()
            }
            Pieces::Queen(_queen) => {
                let validator = QueenValidator::new(base_location);
                validator.validate() && validator.can_move(&board)
            }
            Pieces::Rook(_rook_info) => {
                let validator = RookValidator::new(base_location);
                validator.validate() && validator.can_move(&board)
            }
            Pieces::Knight(_knight) => KnightValidator::new(base_location).validate(),
        };
        if is_valid {
            ValidMovement::VALID
        } else {
            ValidMovement::INVALID
        }
    }
}
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// pub trait PieceClone {
//     fn clone_box(&self) -> Box<dyn Piece>;
// }
//
// impl<T> PieceClone for T
// where
//     T: 'static + Piece + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Piece> {
//         Box::new(self.clone())
//     }
// }
//
// impl Clone for Box<dyn Piece> {
//     fn clone(&self) -> Box<dyn Piece> {
//         self.clone_box()
//     }
// }
