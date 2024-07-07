use crate::pieces::color::Color;
use crate::pieces::piece::{PieceInfo, PieceType, Pieces};

static EMPTY: &'static str = "  ";
static FILLED: &'static str = "##";

#[derive(Clone)]
pub enum Square {
    // TODO: We shouldn't be using Refcell
    White(Option<Pieces>),
    Black(Option<Pieces>),
}
impl Square {
    pub fn has_piece(&self) -> bool {
        match self {
            Square::White(piece) => match piece {
                None => false,
                Some(_) => true,
            },
            Square::Black(piece) => match piece {
                None => false,
                Some(_) => true,
            },
        }
    }

    pub fn get_piece_type(&self) -> Option<PieceType> {
        match self {
            Square::White(piece) => match piece {
                None => None,
                Some(piece) => Some(piece.get_piece_type().clone()),
            },
            Square::Black(piece) => match piece {
                None => None,
                Some(piece) => Some(piece.get_piece_type().clone()),
            },
        }
    }

    pub fn get_piece_info(&self) -> Option<PieceInfo> {
        match self {
            Square::White(piece) => match piece {
                None => None,
                Some(piece) => Some(piece.get_piece_info_clone()),
            },
            Square::Black(piece) => match piece {
                None => None,
                Some(piece) => Some(piece.get_piece_info_clone()),
            },
        }
    }

    pub fn print(&self) -> String {
        match self {
            Square::White(piece) => match piece {
                None => EMPTY.to_string(),
                Some(p) => p.get_name(),
            },
            Square::Black(piece) => match piece {
                None => FILLED.to_string(),
                Some(p) => p.get_name(),
            },
        }
    }

    pub fn get_piece(&self) -> &Option<Pieces> {
        match self {
            Square::White(piece) => piece,
            Square::Black(piece) => piece,
        }
    }

    pub fn get_color_of_piece(&self) -> Option<Color> {
        match self {
            Square::White(Some(piece)) => Some(piece.get_color().clone()),
            Square::Black(Some(piece)) => Some(piece.get_color().clone()),
            _ => None,
        }
    }

    pub fn get_actual_piece(&mut self) -> Option<Pieces> {
        match self {
            Square::White(ref mut piece) => piece.take(),
            Square::Black(ref mut piece) => piece.take(),
        }
    }

    // TODO: Probably not needed.
    fn get_piece_reference(&self) -> Option<&Pieces> {
        match self {
            Square::White(Some(piece)) => Some(piece),
            Square::Black(Some(piece)) => Some(piece),
            _ => None,
        }
    }

    pub fn set_piece(&mut self, new_piece: Pieces) {
        match self {
            Square::White(piece) => *piece = Some(new_piece),
            Square::Black(piece) => *piece = Some(new_piece),
        }
    }

    pub fn remove_piece(&mut self) {
        match self {
            Square::White(piece) => *piece = None,
            Square::Black(piece) => *piece = None,
        }
    }
}
