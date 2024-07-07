use std::collections::HashMap;

use crate::game::square::Square;
use crate::pieces::color::Color;
use crate::pieces::piece::{PieceInfo, Pieces};

// the board will follow the design of wiki.
pub struct Board {
    pub squares: Vec<Vec<Square>>,
    // keeps track of how many pieces we have.
    white_pieces: HashMap<String, u8>,
    black_pieces: HashMap<String, u8>,
}

impl Board {
    const COLUMNS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    const ROWS: [u8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];

    fn create_piece_info(should_be_black: bool, track_movement: bool) -> PieceInfo {
        let color = if should_be_black {
            Color::Black
        } else {
            Color::White
        };
        PieceInfo::new(color, track_movement)
    }

    pub fn maybe_get_piece(row: usize, column: usize) -> Option<Pieces> {
        if row == 0 || row == 7 {
            // TODO: UGLY!!
            let should_be_black = row == 0;
            let piece = match Self::COLUMNS[column] {
                'a' | 'h' => Some(Pieces::Rook(Board::create_piece_info(
                    should_be_black,
                    true,
                ))),
                'b' | 'g' => Some(Pieces::Knight(Board::create_piece_info(
                    should_be_black,
                    false,
                ))),
                'c' | 'f' => Some(Pieces::Bishop(Board::create_piece_info(
                    should_be_black,
                    false,
                ))),
                'd' => Some(Pieces::Queen(Board::create_piece_info(
                    should_be_black,
                    false,
                ))),
                'e' => Some(Pieces::King(Board::create_piece_info(
                    should_be_black,
                    true,
                ))),
                _ => None,
            };
            return match piece {
                None => None,
                Some(p) => Some(p),
            };
        } else if row == 1 || row == 6 {
            let should_be_black = row == 1;
            let piece = Pieces::Pawn(Board::create_piece_info(should_be_black, true));
            return Some(piece);
        }
        return None;
    }

    pub fn create_empty_board() -> Board {
        let mut squares = vec![
            vec![Square::White(None); 8]; // Create a 8x8 grid of None squares initially
            8
        ];
        for row in 0..squares.len() {
            let white_is_first = row % 2 == 0;
            for column in 0..squares[row].len() {
                let be_black: bool;
                if white_is_first {
                    be_black = column % 2 == 1;
                } else {
                    be_black = column % 2 == 0;
                }
                if be_black {
                    squares[row][column] = Square::Black(None);
                } else {
                    squares[row][column] = Square::White(None);
                }
            }
        }
        Board {
            squares,
            white_pieces: HashMap::new(),
            black_pieces: HashMap::new(),
        }
    }

    pub fn new() -> Board {
        let mut squares = vec![
            vec![Square::White(None); 8]; // Create a 8x8 grid of None squares initially
            8
        ];
        // keep track of the pieces
        let pairs = [('R', 2), ('N', 2), ('B', 2), ('Q', 1), ('K', 1), ('P', 8)];
        let white_pieces = HashMap::from(pairs.map(|p| (format!("W{}", p.0), p.1 as u8)));
        let black_pieces = HashMap::from(pairs.map(|p| (format!("B{}", p.0), p.1 as u8)));
        for row in 0..squares.len() {
            let white_is_first = row % 2 == 0;
            for column in 0..squares[row].len() {
                let be_black: bool;
                if white_is_first {
                    be_black = column % 2 == 1;
                } else {
                    be_black = column % 2 == 0;
                }
                if be_black {
                    squares[row][column] = Square::Black(Board::maybe_get_piece(row, column));
                } else {
                    squares[row][column] = Square::White(Board::maybe_get_piece(row, column));
                }
            }
        }
        Board {
            squares,
            white_pieces,
            black_pieces,
        }
    }

    fn decrement(&self, ref_count: Option<&mut u8>) {
        match ref_count {
            None => {}
            Some(ref_count) => {
                *ref_count -= 1;
            }
        }
    }

    pub fn remove_piece_from_map(&mut self, piece: Pieces) {
        let lambda = |map: &mut HashMap<String, u8>, key: &str| -> () {
            match map.get_mut(key) {
                Some(count) => {
                    *count -= 1;
                    if *count == 0 {
                        map.remove(key);
                    }
                }
                None => {}
            }
        };
        match piece.get_color() {
            Color::White => {
                let map = &mut self.white_pieces;
                lambda(map, &piece.get_name());
            }
            Color::Black => {
                let map = &mut self.black_pieces;
                lambda(map, &piece.get_name());
            }
        }
    }

    /// Given a row and column, replace the piece in the square with the one passed.
    /// If the square had a piece, we'll return it.
    pub fn set_piece(&mut self, row: u8, column: u8, piece: Pieces) {
        let to_square = &mut self.squares[row as usize][column as usize];
        to_square.set_piece(piece)
    }

    pub fn create_new_piece_and_set_as_moved(
        &self,
        c: char,
        should_be_black: bool,
    ) -> Option<Pieces> {
        let option_piece = match c.to_ascii_uppercase() {
            'Q' => Some(Pieces::Queen(Board::create_piece_info(
                should_be_black,
                true,
            ))),
            'N' => Some(Pieces::Knight(Board::create_piece_info(
                should_be_black,
                false,
            ))),
            'B' => Some(Pieces::Bishop(Board::create_piece_info(
                should_be_black,
                false,
            ))),
            'R' => Some(Pieces::Rook(Board::create_piece_info(
                should_be_black,
                false,
            ))),
            _ => None,
        };
        match option_piece {
            None => None,
            Some(mut piece) => {
                piece.set_as_moved();
                Some(piece)
            }
        }
    }

    pub fn remove_piece(&mut self, row: u8, column: u8) {
        let to_square = &mut self.squares[row as usize][column as usize];
        to_square.remove_piece()
    }
    /// Simply prints the alpha for a board' column
    ///
    /// # Arguments
    ///
    /// * `column`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn print_alpha(&self, column: &usize) {
        if *column == 0 || *column == self.squares.len() + 1 {
            print!("  ")
        } else {
            print!(" {}", Board::COLUMNS[column - 1]);
        }
    }

    pub fn print_all(&self) {
        for row in 0..self.squares.len() + 2 {
            for column in 0..self.squares.len() + 2 {
                if row == 0 {
                    self.print_alpha(&column);
                } else if row == self.squares.len() + 1 {
                    self.print_alpha(&column);
                } else {
                    let adjusted_row = row - 1;
                    if column == 0 {
                        print!("{} ", Board::ROWS[adjusted_row])
                    } else if column == self.squares.len() + 1 {
                        print!(" {}", Board::ROWS[adjusted_row])
                    } else {
                        let square = &self.squares[adjusted_row][column - 1];
                        print!("{}", square.print());
                    }
                }
            }
            println!();
        }
    }
}
