use crate::game::board::Board;
use crate::game::check_mate_status::CheckMateStatus;
use crate::pieces::color::Color;
use crate::pieces::piece::{PieceType, Pieces, ValidMovement};
use crate::pieces::validator::base_validator::ContainsConflictingPiece;
use crate::pieces::validator::row_column::{BaseLocation, RowColumn};
use std::any::Any;
use std::collections::VecDeque;
use std::io;

pub static ALL_MOVEMENTS: [[i8; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[derive(Debug)]
pub enum Errors {
    InvalidInput,
}
#[derive(Eq, PartialEq)]
enum State {
    Playing,
    Ended,
}
impl State {
    fn has_ended(&self) -> bool {
        match *self {
            State::Ended => true,
            _ => false,
        }
    }
}
#[derive(Eq, PartialEq, Debug)]
pub struct History {
    color: Color,
    from: RowColumn,
    to: RowColumn,
    piece_type: PieceType,
}
impl History {
    fn new(color: &Color, from: &RowColumn, to: &RowColumn, piece_type: &PieceType) -> History {
        History {
            color: color.clone(),
            from: from.clone(),
            to: to.clone(),
            piece_type: piece_type.clone(),
        }
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_from(&self) -> &RowColumn {
        &self.from
    }

    pub fn get_to(&self) -> &RowColumn {
        &self.to
    }
}

pub struct HistoryOfLastFiveMovement {
    history: VecDeque<History>,
}

impl HistoryOfLastFiveMovement {
    pub fn new() -> HistoryOfLastFiveMovement {
        HistoryOfLastFiveMovement {
            history: VecDeque::new(),
        }
    }

    pub fn add_history(&mut self, history: History) {
        if self.history.len() >= 5 {
            self.history.pop_front();
        }
        self.history.push_back(history)
    }

    pub fn peek(&self) -> Option<&History> {
        if self.history.is_empty() {
            return None;
        }
        self.history.get(self.history.len() - 1)
    }

    pub fn size(&self) -> usize {
        self.history.len()
    }
}

pub struct Game {
    board: Board,
    state: State,
    current_color: Color,
    history: HistoryOfLastFiveMovement,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();
        Game {
            board,
            state: State::Playing,
            current_color: Color::White,
            history: HistoryOfLastFiveMovement::new(),
        }
    }

    fn print_help(&self) {
        println!("You have the following commands:");
        println!("move [row_column] [row_column]: Moves a piece from an area to another.");
        println!("     e.g.: a2 a4");
        println!("     If invalid(piece missing, can't move, etc), we'll re-request your input.");
        println!("exit: Exit the game.");
        println!("help: prints this.");
        println!();
    }

    /// Expects a string that's alphanumeric.
    /// The first char must be between a -h(column).
    /// The second char must 1-8(row).
    /// If any of them are invalid, we'll return an error.
    ///
    /// # Arguments
    ///
    /// * `input`: A reference to the input.
    ///
    /// returns: Result<(u8, u8), String>
    ///
    /// # Examples
    ///
    ///
    /// 'a8' // returns 0(column), 7(row)
    /// 'h8' // returns 7(column), 7(row)
    ///
    fn valid_input(&self, input: &str) -> Result<(u8, u8, Option<char>), String> {
        // must be length 2
        let trimmed = input.trim();
        let mut chars = trimmed.chars();
        if let (Some(char), Some(num_char)) = (&chars.next(), &chars.next()) {
            let difference = (*char as u8) - 'a' as u8;
            if difference > 7 {
                return Err(String::from("Must be a char between a - h."));
            }
            let num: u8 = *num_char as u8 - '0' as u8;
            if num > 8 || num == 0 {
                return Err(String::from("Invalid number. Must be between 1-8"));
            }
            let next_char = chars.next();
            Ok((difference, num - 1, next_char))
        } else {
            return Err(String::from("Invalid input."));
        }
    }

    /// When we read the row input, we need to convert it to our 2d array
    ///
    /// # Arguments
    ///
    /// * `row`:
    ///
    /// returns: u8
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn convert_row_to_our_array(&self, row: &u8) -> Result<u8, Errors> {
        if *row > 8u8 {
            return Err(Errors::InvalidInput);
        }
        Ok(7 - *row)
    }

    fn is_current_player_color(game_color: &Color, current_color: &Color) -> bool {
        let matches = match game_color {
            Color::White => matches!(current_color, Color::White),
            Color::Black => matches!(current_color, Color::Black),
        };
        return matches;
    }

    fn self_is_current_player_color<'a>(&'a self, current_color: &'a Color) -> bool {
        let matches = match &self.current_color {
            Color::White => matches!(current_color, Color::White),
            Color::Black => matches!(current_color, Color::Black),
        };
        return matches;
    }

    fn set_next_player_color(&mut self) {
        let game_color = &self.current_color;
        let next_color = match game_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        self.current_color = next_color;
    }

    fn set_piece_to_square(&mut self, row_column: &RowColumn, piece: Pieces) {
        self.board
            .set_piece(row_column.row, row_column.column, piece);
    }

    fn remove_piece_from_square(&mut self, row_column: &RowColumn) {
        self.board.remove_piece(row_column.row, row_column.column);
    }

    fn is_color_square_current_user_color(&self, row: usize, column: usize) -> bool {
        if let Some(color) = self.board.squares[row][column].get_color_of_piece() {
            if self.self_is_current_player_color(&color) {
                return true;
            }
        }
        false
    }

    fn is_color_square_matched(&self, row: usize, column: usize, color: &Color) -> bool {
        if let Some(piece_color) = self.board.squares[row][column].get_color_of_piece() {
            return match &piece_color {
                Color::White => matches!(color, Color::White),
                Color::Black => matches!(color, Color::Black),
            };
        }
        false
    }
    fn get_king_position(&self) -> Option<RowColumn> {
        for row in 0..self.board.squares.len() {
            for column in 0..self.board.squares[0].len() {
                if self.is_color_square_current_user_color(row, column) {
                    continue;
                }
                if let Some(piece_info) = self.board.squares[row][column].get_piece_type() {
                    match piece_info {
                        PieceType::King => return Some(RowColumn::new(row as u8, column as u8)),
                        _ => {
                            // just ignore
                        }
                    }
                }
            }
        }
        return None;
    }

    fn get_movement_around(&self, row_column: &RowColumn) -> Vec<RowColumn> {
        let mut surrounding_area = vec![];
        for movement in ALL_MOVEMENTS {
            let row = row_column.row as i8 + movement[0];
            let column = row_column.column as i8 + movement[1];
            if row < 0 || column < 0 || row >= 8 || column >= 8 {
                continue;
            }
            if self.board.squares[row as usize][column as usize].has_piece() {
                continue;
            }
            surrounding_area.push(RowColumn::new(row as u8, column as u8))
        }
        surrounding_area
    }

    fn is_check(
        &self,
        curr_location: &RowColumn,
        skip_same_color: bool,
        king_color: &Color,
    ) -> Option<RowColumn> {
        for row in 0..self.board.squares.len() {
            for column in 0..self.board.squares[0].len() {
                let is_same_color = self.is_color_square_matched(row, column, king_color);
                if skip_same_color {
                    if is_same_color {
                        continue;
                    }
                } else {
                    if !is_same_color {
                        continue;
                    }
                }
                if curr_location.row as usize == row && curr_location.column as usize == column {
                    continue;
                }
                let row_column = RowColumn::new(row as u8, column as u8);
                match self.board.squares[row][column].get_piece() {
                    None => {}
                    Some(piece_ref) => {
                        match piece_ref.is_valid_movement_has_piece_override(
                            &row_column,
                            &curr_location,
                            &self.board,
                            self.history.peek(),
                            // for pawn
                            true,
                        ) {
                            ValidMovement::VALID
                            | ValidMovement::CASTLING(_)
                            | ValidMovement::EnPassant(_)
                            | ValidMovement::Promotion => return Some(row_column),
                            ValidMovement::INVALID => {}
                        }
                    }
                }
            }
        }
        None
    }

    fn come_to_rescue(&self, target_location: &RowColumn, king_location: &RowColumn) -> bool {
        let movements_between = match self.board.squares[king_location.row as usize]
            [king_location.column as usize]
            .get_piece()
        {
            None => {
                vec![]
            }
            Some(p) => {
                match self.board.squares[target_location.row as usize]
                    [target_location.column as usize]
                    .get_piece_type()
                {
                    None => vec![],
                    Some(t) => match t {
                        PieceType::Knight | PieceType::King => {
                            vec![]
                        }
                        _ => p.get_movements_between(target_location, king_location),
                    },
                }
            }
        };
        for row in 0..self.board.squares.len() {
            for column in 0..self.board.squares[0].len() {
                if self.is_color_square_current_user_color(row, column) {
                    continue;
                }
                if target_location.row as usize == row && target_location.column as usize == column
                {
                    continue;
                }
                let row_column = RowColumn::new(row as u8, column as u8);
                match self.board.squares[row][column].get_piece() {
                    None => {}
                    Some(piece_ref) => {
                        match piece_ref.get_piece_type() {
                            PieceType::King => continue,
                            _ => {}
                        }
                        // TODO: CAN IT BLOCK?
                        match piece_ref.is_valid_movement(
                            &row_column,
                            &target_location,
                            &self.board,
                            self.history.peek(),
                        ) {
                            ValidMovement::VALID => return true,
                            ValidMovement::INVALID | _ => {
                                match piece_ref.get_piece_type() {
                                    PieceType::King => continue,
                                    _ => {}
                                }
                                for movement in &movements_between {
                                    match piece_ref.is_valid_movement(
                                        &row_column,
                                        &movement,
                                        &self.board,
                                        self.history.peek(),
                                    ) {
                                        ValidMovement::VALID => return true,
                                        ValidMovement::INVALID | _ => continue,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn check_check_status(&mut self) -> CheckMateStatus {
        if let Some(king_location) = self.get_king_position() {
            self.print_board();
            let surrounding_area = self.get_movement_around(&king_location);
            let king_color = self.board.squares[king_location.row as usize]
                [king_location.column as usize]
                .get_color_of_piece()
                .unwrap();
            // is the king under fire?
            match self.is_check(&king_location, true, &king_color) {
                None => CheckMateStatus::NONE,
                Some(target) => {
                    // Is there a piece that can attack the location
                    if self.come_to_rescue(&target, &king_location) {
                        return CheckMateStatus::CHECK;
                    }
                    // we remove the king piece to pretend it's not there so we can do the check
                    let king_piece = self.board.squares[king_location.row as usize]
                        [king_location.column as usize]
                        .get_actual_piece()
                        .unwrap();
                    for other in &surrounding_area {
                        if self.is_color_square_current_user_color(
                            other.row as usize,
                            other.column as usize,
                        ) {
                            continue;
                        }
                        match self.is_check(other, true, &king_color) {
                            None => {
                                self.board.squares[king_location.row as usize]
                                    [king_location.column as usize]
                                    .set_piece(king_piece);
                                return CheckMateStatus::CHECK;
                            }
                            Some(target) => {
                                if self.come_to_rescue(&target, &other) {
                                    self.board.squares[king_location.row as usize]
                                        [king_location.column as usize]
                                        .set_piece(king_piece);
                                    return CheckMateStatus::CHECK;
                                }
                            }
                        }
                    }

                    self.board.squares[king_location.row as usize][king_location.column as usize]
                        .set_piece(king_piece);
                    CheckMateStatus::CHECKMATE
                }
            }
        } else {
            CheckMateStatus::NONE
        }
    }

    ///
    /// First, check if the from square actually has a piece.
    /// If a piece exists, we check if that piece can actually go to the square.
    /// Finally, if valid, replace the piece.
    fn start_movement(
        &mut self,
        from: &RowColumn,
        to: &RowColumn,
        next_piece: Option<char>,
    ) -> Result<(), String> {
        let from_square = &mut self.board.squares[from.row as usize][from.column as usize];
        let piece_type: Option<PieceType>;
        match from_square.get_actual_piece() {
            None => return Err(String::from("Must contain a piece")),
            Some(mut piece) => {
                piece_type = Some(piece.get_piece_type().clone());
                // TODO: Check if it's actually a piece that belongs to the controlling user.
                let is_valid =
                    piece.is_valid_movement(&from, &to, &self.board, self.history.peek());
                match is_valid {
                    ValidMovement::INVALID => {
                        self.set_piece_to_square(&from, piece);
                        return Err(String::from("Invalid movement."));
                    }
                    ValidMovement::VALID => {
                        piece.set_as_moved();
                        self.set_piece_to_square(&to, piece);
                    }
                    ValidMovement::CASTLING(location) => {
                        piece.set_as_moved();
                        self.set_piece_to_square(&to, piece);
                        if let Some(mut rook) = self.board.squares[location.from.row as usize]
                            [location.from.column as usize]
                            .get_actual_piece()
                        {
                            rook.set_as_moved();
                            self.set_piece_to_square(&location.to, rook);
                        } else {
                            panic!("bug!! Rook should have been around!!")
                        }
                    }
                    ValidMovement::EnPassant(row_column) => {
                        self.remove_piece_from_square(&row_column);
                        piece.set_as_moved();
                        self.set_piece_to_square(&to, piece);
                    }
                    ValidMovement::Promotion => {
                        // piece should be deleted.
                        if let Some(user_wanted_piece) = next_piece {
                            let some_new_piece = self.board.create_new_piece_and_set_as_moved(
                                user_wanted_piece,
                                matches!(self.current_color, Color::Black),
                            );
                            match some_new_piece {
                                None => {
                                    self.set_piece_to_square(&from, piece);
                                    return Err(String::from(
                                        "Invalid piece. Please enter Q, R, B, or N",
                                    ));
                                }
                                Some(new_piece) => {
                                    // TODO: I don' think i need to call the drop fun
                                    // drop(piece);
                                    self.set_piece_to_square(&to, new_piece);
                                }
                            }
                        } else {
                            self.set_piece_to_square(&from, piece);
                            return Err(String::from("We need to know what piece you want"));
                        }
                    }
                };
            }
        }
        // add the history.
        self.history.add_history(History::new(
            &self.current_color,
            from,
            to,
            &piece_type.unwrap(),
        ));
        match self.check_check_status() {
            CheckMateStatus::NONE => {
                // do nothing
            }
            CheckMateStatus::CHECK => {
                println!("Check!")
            }
            CheckMateStatus::CHECKMATE => {
                println!("Checkmate! {} wins!", self.current_color.get_color());
                self.state = State::Ended;
            }
        }
        self.set_next_player_color();
        Ok(())
    }

    fn get_current_user_color(&self) -> String {
        return self.current_color.get_color();
    }

    ///
    /// given an iterator that references a string,
    /// takes the first two, if any, and validates the input.
    /// Once validated, will return the RowColumn;; these values will be adjusted.
    /// # Arguments
    ///
    /// * `iterator`:
    ///
    /// returns: Result<(), String>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn validate_input(
        &self,
        mut iterator: std::str::Split<&str>,
    ) -> Result<(BaseLocation, Option<char>), String> {
        let from_op = iterator.next();
        let to_op = iterator.next();

        if let (Some(from), Some(to)) = (from_op, to_op) {
            match (self.valid_input(from), self.valid_input(to)) {
                (Ok((from_column, from_row, _)), Ok((to_column, to_row, piece_option))) => {
                    // convert the rows to our array.
                    let adjusted_from_row = self.convert_row_to_our_array(&from_row).unwrap();
                    let adjusted_to_row = self.convert_row_to_our_array(&to_row).unwrap();
                    // use a reference of from square. We won't
                    let from_square =
                        &self.board.squares[adjusted_from_row as usize][from_column as usize];
                    // if not an actual piece, we can't mutate it.
                    return if let Some(color) = from_square.get_color_of_piece() {
                        let matches = self.self_is_current_player_color(&color); // Game::is_current_player_color(&self.current_color, color);
                        if !matches {
                            return Err("You can't move the other's person piece".to_string());
                        }

                        // now we're actually going to mutate it since we've confirmed it's a valid user piece.
                        return Ok((
                            BaseLocation::new_row_column(
                                RowColumn::new(adjusted_from_row, from_column),
                                RowColumn::new(adjusted_to_row, to_column),
                            ),
                            piece_option,
                        ));
                    } else {
                        Err("No piece! Try again".to_string())
                    };
                }
                // TODO: Clean this up.
                (Err(error), Err(error_two)) => Err(vec![error, error_two].join(",")),
                (Ok(_), Err(error_two)) => Err(error_two),
                (Err(error), Ok(_)) => Err(error),
            }
        } else {
            Err(String::from("Missing from and/or to"))
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `input`:
    ///
    /// returns: Result<(), String>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn handle_input(&mut self, input: &str) -> Result<(), String> {
        let mut splitted = input.split(" ");
        match splitted.next() {
            None => Err(String::from("Invalid input. Will just move on")),
            Some(action) => match action {
                "move" => match self.validate_input(splitted) {
                    Ok((location_info, next_piece_optional_char)) => {
                        match self.start_movement(
                            &location_info.from,
                            &location_info.to,
                            next_piece_optional_char,
                        ) {
                            Ok(_) => {
                                if self.state == State::Playing {
                                    println!(
                                        "{}, it's your turn now",
                                        self.get_current_user_color()
                                    );
                                }
                                Ok(())
                            }
                            Err(error) => Err(error),
                        }
                    }
                    Err(error) => Err(error),
                },
                "help" => {
                    self.print_help();
                    Ok(())
                }
                "exit" => {
                    self.state = State::Ended;
                    Ok(())
                }
                _ => Err(format!("Unknown input {}", action)),
            },
        }
    }

    pub fn set_piece(&mut self, row: u8, column: u8, piece: Pieces) {
        self.board.set_piece(row, column, piece);
    }

    pub fn set_piece_row_col(&mut self, row_column: &RowColumn, piece: Pieces) {
        self.board
            .set_piece(row_column.row, row_column.column, piece);
    }

    pub fn print_board(&self) {
        self.board.print_all();
    }

    pub fn read_input(&mut self) {
        // white moves first
        println!("Welcome!");
        println!("Type in help if you're new to this console game.");
        println!("White moves first. Waiting on next action...");
        while !self.state.has_ended() {
            self.board.print_all();
            let mut user_input = String::new();

            match io::stdin().read_line(&mut user_input) {
                Err(_) => {
                    print!("Failed to read input. try again")
                }
                Ok(_) => match self.handle_input(&user_input.trim()) {
                    Err(error) => {
                        println!("{}", error)
                    }
                    Ok(_) => {}
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::game::board::Board;
    use crate::game::game::{Game, History, HistoryOfLastFiveMovement, State};
    use crate::pieces::color::Color;
    use crate::pieces::piece::{PieceInfo, PieceType, Pieces};
    use crate::pieces::validator::row_column::RowColumn;

    impl Game {
        pub fn new_test() -> Game {
            let board = Board::create_empty_board();
            Game {
                board,
                state: State::Playing,
                current_color: Color::White,
                history: HistoryOfLastFiveMovement::new(),
            }
        }

        pub fn inject_board(board: Board) -> Game {
            Game {
                board,
                state: State::Playing,
                current_color: Color::White,
                history: HistoryOfLastFiveMovement::new(),
            }
        }
    }

    #[test]
    fn test_a3() {
        let game = Game::new();
        let movement = String::from("a3");
        let result = game.valid_input(&movement);
        assert_eq!(result, Ok((0, 2, None)))
    }

    #[test]
    fn test_h_8() {
        let game = Game::new();
        let movement = String::from("h8");
        let result = game.valid_input(&movement);
        assert_eq!(result, Ok((7, 7, None)))
    }

    #[test]
    fn invalid_input_h9() {
        let game = Game::new();
        let movement = String::from("h9");
        let result = game.valid_input(&movement);
        assert_eq!(
            result,
            Err(String::from("Invalid number. Must be between 1-8"))
        )
    }

    #[test]
    fn invalid_input_h0() {
        let game = Game::new();
        let movement = String::from("h0");
        let result = game.valid_input(&movement);
        assert_eq!(
            result,
            Err(String::from("Invalid number. Must be between 1-8"))
        )
    }

    #[test]
    fn invalid_input_i0() {
        let game = Game::new();
        let movement = String::from("i0");
        let result = game.valid_input(&movement);
        assert_eq!(result, Err(String::from("Must be a char between a - h.")))
    }

    #[test]
    fn check_conversion() {
        let game = Game::new();
        for (movement, expected) in [
            (String::from("a1"), 7u8),
            (String::from("a2"), 6u8),
            (String::from("a3"), 5u8),
            (String::from("a4"), 4u8),
            (String::from("a5"), 3u8),
            (String::from("a6"), 2u8),
            (String::from("a7"), 1u8),
        ] {
            let (_, from_row, _) = game.valid_input(&movement).unwrap();
            let converted_row = game.convert_row_to_our_array(&from_row).unwrap();
            assert_eq!(converted_row, expected);
        }
    }

    fn get_row_column(game: &Game, input: String) -> RowColumn {
        let (from_column, from_row, _) = game.valid_input(&input).unwrap();
        let converted_row = game.convert_row_to_our_array(&from_row).unwrap();
        return RowColumn {
            row: converted_row,
            column: from_column,
        };
    }

    #[test]
    fn checking_white_movement_pawn_should_be_able_to_move_up() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   WP  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // b2 should be able to move to be.
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("b2"));
        let to = get_row_column(&game, String::from("b3"));
        let pawn = Pieces::Pawn(PieceInfo::new(Color::White, false));
        game.set_piece(from.row, from.column, pawn);
        let result = game.start_movement(&from, &to, None);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn white_pawn_should_not_move_diagonally() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   WP  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // b2 should be able to move to be.
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("b2"));
        let to = get_row_column(&game, String::from("c3"));
        let pawn = Pieces::Pawn(PieceInfo::new(Color::White, false));
        game.set_piece(from.row, from.column, pawn);
        let result = game.start_movement(&from, &to, None);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn white_pawn_should_be_able_to_take_over() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  BP  ##  ##   3
        // 2   WP  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // b2 should be able to move to be.
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("b2"));
        let to = get_row_column(&game, String::from("c3"));
        let pawn = Pieces::Pawn(PieceInfo::new(Color::White, true));
        let black_pawn = Pieces::Pawn(PieceInfo::new(Color::Black, true));
        game.set_piece(from.row, from.column, pawn);
        game.set_piece(to.row, to.column, black_pawn);
        let result = game.start_movement(&from, &to, None);

        assert_eq!(result.is_ok(), true);
        game.print_board();
    }

    #[test]
    fn white_pawn_in_front_blocking_piece() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##BP##  ##  ##   3
        // 2   WP  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // the piece in b2 should not be able to move.
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("b2"));
        let to = get_row_column(&game, String::from("b3"));
        let pawn = Pieces::Pawn(PieceInfo::new(Color::White, true));
        let black_pawn = Pieces::Pawn(PieceInfo::new(Color::Black, true));

        game.set_piece(from.row, from.column, pawn);
        game.set_piece(to.row, to.column, black_pawn);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), "Invalid movement.");
    }

    #[test]
    fn white_bishop_cant_jump_over_other_pieces_blocked_by_black_pawn() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  XX  ##   5
        // 4   ##  BP  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   WB  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // the piece in b2 should not be able to move to where the xx is.
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("b2"));
        let black_pawn_position = get_row_column(&game, String::from("d4"));
        let to = get_row_column(&game, String::from("e5"));
        let white_bishop = Pieces::Bishop(PieceInfo::new(Color::White, false));
        let black_pawn = Pieces::Pawn(PieceInfo::new(Color::Black, true));

        game.set_piece(from.row, from.column, white_bishop);
        game.set_piece(
            black_pawn_position.row,
            black_pawn_position.column,
            black_pawn,
        );

        let result = game.start_movement(&from, &to, None);

        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), "Invalid movement.");
    }

    #[test]
    fn white_rook_cant_jump_over_other_pieces_blocked_by_black_pawn() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   WR  BP  XX  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // the piece in b2 should not be able to move to where the xx is.
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("b2"));
        let black_pawn_position = get_row_column(&game, String::from("d2"));
        let to = get_row_column(&game, String::from("f2"));
        let white_rook = Pieces::Rook(PieceInfo::new(Color::White, false));
        let black_pawn = Pieces::Pawn(PieceInfo::new(Color::Black, true));

        game.set_piece(from.row, from.column, white_rook);
        game.set_piece(
            black_pawn_position.row,
            black_pawn_position.column,
            black_pawn,
        );
        game.print_board();

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid movement.");
    }

    #[test]
    fn white_attempts_to_castle_e1_to_g1() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  WK  ##WR1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let white_rook_placement = get_row_column(&game, String::from("h1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));

        let white_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_ok());
        game.print_board();

        let piece_info = &game.board.squares[to.row as usize][to.column as usize]
            .get_piece_info()
            .unwrap();
        assert_eq!(true, piece_info.get_has_moved());
    }

    #[test]
    fn white_attempts_to_castle_e1_to_g1_rook_already_moved() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  WK  ##WR1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let white_rook_placement = get_row_column(&game, String::from("h1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));

        let mut white_rook_movement = PieceInfo::new(Color::White, true);
        white_rook_movement.set_has_moved();
        let white_rook = PieceType::Rook.create_actual_piece(white_rook_movement);

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
        game.print_board()
    }

    #[test]
    fn white_attempts_to_castle_e1_c1() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 WR  ##  WK  ##   1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("c1"));
        let white_rook_placement = get_row_column(&game, String::from("a1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));

        let white_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn white_attempts_to_castle_e1_c1_rook_not_there() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 WR  ##  WK  ##   1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let white_rook_placement = get_row_column(&game, String::from("a1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));

        let white_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
        game.print_board()
    }

    #[test]
    fn white_attempts_to_castle_e1_g1_piece_blocking_h1() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1     ##  WK  WNWR 1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let white_knight_placement = get_row_column(&game, String::from("g1"));
        let white_rook_placement = get_row_column(&game, String::from("h1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));
        let white_knight =
            PieceType::Knight.create_actual_piece(PieceInfo::new(Color::White, true));

        let white_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);
        game.set_piece_row_col(&white_knight_placement, white_knight);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn white_attempts_to_castle_e1_g1_piece_blocking_f1() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  WKWNWNWR 1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let white_knight_placement = get_row_column(&game, String::from("f1"));
        let white_rook_placement = get_row_column(&game, String::from("h1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));
        let white_knight =
            PieceType::Knight.create_actual_piece(PieceInfo::new(Color::White, true));

        let white_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);
        game.set_piece_row_col(&white_knight_placement, white_knight);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn white_attempts_to_castle_e1_a1_piece_blocking_b1() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  WKWNWNWR 1
        //    a b c d e f g h
        // white king is at e1.
        // white rook is at a1 and h1
        // white should be able to perform a castling
        // white king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let white_knight_placement = get_row_column(&game, String::from("b1"));
        let white_rook_placement = get_row_column(&game, String::from("a1"));
        let white_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::White, true));
        let white_knight =
            PieceType::Knight.create_actual_piece(PieceInfo::new(Color::White, true));

        let white_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);
        game.set_piece_row_col(&white_rook_placement, white_rook);
        game.set_piece_row_col(&white_knight_placement, white_knight);

        let result = game.start_movement(&from, &to, None);

        game.print_board();
        assert_eq!(true, result.is_err());
        game.print_board();
    }

    // BLACK PIECES
    #[test]
    fn black_attempts_to_castle_e8_to_g8() {
        // black king is at e1.
        // black rook is at a1 and h1
        // black should be able to perform a castling
        // black king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e8"));
        let to = get_row_column(&game, String::from("g8"));
        let black_rook_placement = get_row_column(&game, String::from("h8"));
        let black_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::Black, true));

        let black_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::Black, true));

        game.set_piece_row_col(&from, black_king);
        game.set_piece_row_col(&black_rook_placement, black_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_ok());
        game.print_board()
    }

    #[test]
    fn black_attempts_to_castle_e1_c1() {
        // black king is at e1.
        // black rook is at a1 and h1
        // black should be able to perform a castling
        // black king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e8"));
        let to = get_row_column(&game, String::from("c8"));
        let black_rook_placement = get_row_column(&game, String::from("e8"));
        let black_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::Black, true));

        let black_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::Black, true));

        game.set_piece_row_col(&from, black_king);
        game.set_piece_row_col(&black_rook_placement, black_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_ok());
        game.print_board()
    }

    #[test]
    fn black_attempts_to_castle_e1_c1_rook_not_there() {
        // black king is at e1.
        // black rook is at a1 and h1
        // black should be able to perform a castling
        // black king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e8"));
        let to = get_row_column(&game, String::from("g8"));
        let black_rook_placement = get_row_column(&game, String::from("a8"));
        let black_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::Black, true));

        let black_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::Black, true));

        game.set_piece_row_col(&from, black_king);
        game.set_piece_row_col(&black_rook_placement, black_rook);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
        game.print_board()
    }

    #[test]
    fn black_attempts_to_castle_e1_g1_piece_blocking_h1() {
        // black king is at e1.
        // black rook is at a1 and h1
        // black should be able to perform a castling
        // black king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e8"));
        let to = get_row_column(&game, String::from("g8"));
        let black_knight_placement = get_row_column(&game, String::from("g8"));
        let black_rook_placement = get_row_column(&game, String::from("h8"));
        let black_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_knight =
            PieceType::Knight.create_actual_piece(PieceInfo::new(Color::Black, true));

        let black_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::Black, true));

        game.set_piece_row_col(&from, black_king);
        game.set_piece_row_col(&black_rook_placement, black_rook);
        game.set_piece_row_col(&black_knight_placement, black_knight);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn black_attempts_to_castle_e1_g1_piece_blocking_f1() {
        // black king is at e1.
        // black rook is at a1 and h1
        // black should be able to perform a castling
        // black king will perform e1 to g1
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e8"));
        let to = get_row_column(&game, String::from("g8"));
        let black_knight_placement = get_row_column(&game, String::from("f8"));
        let black_rook_placement = get_row_column(&game, String::from("h8"));
        let black_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_knight =
            PieceType::Knight.create_actual_piece(PieceInfo::new(Color::Black, true));

        let black = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::Black, true));

        game.set_piece_row_col(&from, black_king);
        game.set_piece_row_col(&black_rook_placement, black);
        game.set_piece_row_col(&black_knight_placement, black_knight);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn black_attempts_to_castle_e1_a1_piece_blocking_b1() {
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("e1"));
        let to = get_row_column(&game, String::from("g1"));
        let black_knight_placement = get_row_column(&game, String::from("b1"));
        let black_rook_placement = get_row_column(&game, String::from("a1"));
        let black_king = PieceType::King.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_knight =
            PieceType::Knight.create_actual_piece(PieceInfo::new(Color::Black, true));

        let black_rook = PieceType::Rook.create_actual_piece(PieceInfo::new(Color::Black, true));

        game.set_piece_row_col(&from, black_king);
        game.set_piece_row_col(&black_rook_placement, black_rook);
        game.set_piece_row_col(&black_knight_placement, black_knight);

        let result = game.start_movement(&from, &to, None);

        game.print_board();
        assert_eq!(true, result.is_err());
        game.print_board();
    }

    #[test]
    fn assure_last_history_working_great() {
        let mut histories = HistoryOfLastFiveMovement::new();
        assert_eq!(histories.peek(), None);
        let color = Color::Black;
        let piece_type = &PieceType::Pawn;
        for i in 1..=5 {
            let from = RowColumn::new(i, 0);
            let history = History::new(&color, &from, &from, piece_type);
            histories.add_history(history);
            assert_eq!(
                histories.peek(),
                Some(&History::new(&color, &from, &from, piece_type))
            );
            assert_eq!(histories.size(), i as usize)
        }
        let from = RowColumn::new(6, 0);
        let history = History::new(&color, &from, &from, piece_type);
        histories.add_history(history);
        assert_eq!(
            histories.peek(),
            Some(&History::new(&color, &from, &from, piece_type))
        );
        assert_eq!(histories.size(), 5)
    }

    struct TestInfo {
        piece_name: String,
        from: RowColumn,
        to: RowColumn,
    }
    #[test]
    fn en_passant_test() {
        // TODO: Check invalid movements.4
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WP  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 2 WP - f2 f4
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  WP  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 3 BP - c7 to c5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  ##  ##   5
        // 4   ##  ##  WP  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 4 wp - f4 -f5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  ##WP##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 5 bp - e7 -e5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  BPWP##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // en passant WP - f5 e6 - should be valid
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##WP##  ## 6
        // 5 ##  BP  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h

        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);

        let white_pawn_start = get_row_column(&game, String::from("f2"));
        let black_pawn_1_start = get_row_column(&game, String::from("e7"));
        let black_pawn_2_start = get_row_column(&game, String::from("c7"));
        let black_pawn_1 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_pawn_2 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let white_pawn = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&white_pawn_start, white_pawn);
        game.set_piece_row_col(&black_pawn_1_start, black_pawn_1);
        game.set_piece_row_col(&black_pawn_2_start, black_pawn_2);

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f2")),
            /*to=*/ &get_row_column(&game, String::from("f4")),
            None,
        )
        .unwrap();

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("c7")),
            /*to=*/ &get_row_column(&game, String::from("c5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f4")),
            /*to=*/ &get_row_column(&game, String::from("f5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("e7")),
            /*to=*/ &get_row_column(&game, String::from("e5")),
            None,
        );

        let result = game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f5")),
            /*to=*/ &get_row_column(&game, String::from("e6")),
            None,
        );

        assert_eq!(true, result.is_ok());

        game.print_board()
    }

    #[test]
    fn en_passant_black_test() {
        // TODO: Check invalid movements.4
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WPWP## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 2 WP - g2 g4
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##WP## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WP  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 3 BP - e7 to e5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  BP  ##   5
        // 4   ##  ##  ##WP## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WP  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 4 wp - g4 -g5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  BP  WP   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WP  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 5 bp - e5 -e4
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  ##  WP   5
        // 4   ##  ##BP##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WP  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 6 WP - f2 f4
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  ##  ##   5
        // 4   ##  ##BPWP  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h

        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);

        let white_pawn_1_start = get_row_column(&game, String::from("f2"));
        let white_pawn_2_start = get_row_column(&game, String::from("g2"));
        let black_pawn_1_start = get_row_column(&game, String::from("e7"));
        let black_pawn_2_start = get_row_column(&game, String::from("c7"));
        let black_pawn_1 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_pawn_2 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let white_pawn_1 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));
        let white_pawn_2 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&white_pawn_1_start, white_pawn_1);
        game.set_piece_row_col(&white_pawn_2_start, white_pawn_2);
        game.set_piece_row_col(&black_pawn_1_start, black_pawn_1);
        game.set_piece_row_col(&black_pawn_2_start, black_pawn_2);

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("g2")),
            /*to=*/ &get_row_column(&game, String::from("g4")),
            None,
        )
        .unwrap();

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("e7")),
            /*to=*/ &get_row_column(&game, String::from("e5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("g4")),
            /*to=*/ &get_row_column(&game, String::from("g5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("e5")),
            /*to=*/ &get_row_column(&game, String::from("e4")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f2")),
            /*to=*/ &get_row_column(&game, String::from("f4")),
            None,
        );
        let result = game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("e4")),
            /*to=*/ &get_row_column(&game, String::from("f3")),
            None,
        );

        assert_eq!(true, result.is_ok());

        game.print_board()
    }

    #[test]
    fn en_passant_test_invalid() {
        // Let's imagine this
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  WP  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 2 WP - f2 f4
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  BP  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  ##  ##  ##   5
        // 4   ##  ##  WP  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 3 BP - c7 to c5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  ##  ##   5
        // 4   ##  ##  WP  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 4 wp - f4 -f5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  BP  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  ##WP##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // part 5 bp - e7 -e5
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  BPWP##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h
        // en passant WP - f5 g6 - should be invalid
        // en passant WP - f5 e4 - should be invalid
        //    a b c d e f g h
        // 8   ##  ##  ##  ## 8
        // 7 ##  ##  ##  ##   7
        // 6   ##  ##  ##  ## 6
        // 5 ##  BP  BPWP##   5
        // 4   ##  ##  ##  ## 4
        // 3 ##  ##  ##  ##   3
        // 2   ##  ##  ##  ## 2
        // 1 ##  ##  ##  ##   1
        //    a b c d e f g h

        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);

        let white_pawn_start = get_row_column(&game, String::from("f2"));
        let black_pawn_1_start = get_row_column(&game, String::from("e7"));
        let black_pawn_2_start = get_row_column(&game, String::from("c7"));
        let black_pawn_1 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_pawn_2 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let white_pawn = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&white_pawn_start, white_pawn);
        game.set_piece_row_col(&black_pawn_1_start, black_pawn_1);
        game.set_piece_row_col(&black_pawn_2_start, black_pawn_2);

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f2")),
            /*to=*/ &get_row_column(&game, String::from("f4")),
            None,
        )
        .unwrap();

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("c7")),
            /*to=*/ &get_row_column(&game, String::from("c5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f4")),
            /*to=*/ &get_row_column(&game, String::from("f5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("e7")),
            /*to=*/ &get_row_column(&game, String::from("e5")),
            None,
        );

        let result = game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f5")),
            /*to=*/ &get_row_column(&game, String::from("g6")),
            None,
        );

        assert_eq!(true, result.is_err());

        let result = game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f5")),
            /*to=*/ &get_row_column(&game, String::from("e4")),
            None,
        );

        assert_eq!(true, result.is_err());

        game.print_board()
    }

    #[test]
    fn en_passant_missed_chance() {
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);

        let white_pawn_start = get_row_column(&game, String::from("f2"));
        let white_another_pawn_start = get_row_column(&game, String::from("g2"));
        let black_pawn_1_start = get_row_column(&game, String::from("e7"));
        let black_pawn_2_start = get_row_column(&game, String::from("c7"));
        let black_pawn_1 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let black_pawn_2 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let white_pawn = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));
        let white_pawn_2 = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&white_pawn_start, white_pawn);
        game.set_piece_row_col(&white_another_pawn_start, white_pawn_2);
        game.set_piece_row_col(&black_pawn_1_start, black_pawn_1);
        game.set_piece_row_col(&black_pawn_2_start, black_pawn_2);

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f2")),
            /*to=*/ &get_row_column(&game, String::from("f4")),
            None,
        )
        .unwrap();

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("c7")),
            /*to=*/ &get_row_column(&game, String::from("c5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f4")),
            /*to=*/ &get_row_column(&game, String::from("f5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("e7")),
            /*to=*/ &get_row_column(&game, String::from("e5")),
            None,
        );

        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("g2")),
            /*to=*/ &get_row_column(&game, String::from("g3")),
            None,
        );

        // pretend black already went
        game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("c5")),
            /*to=*/ &get_row_column(&game, String::from("c4")),
            None,
        );

        let result = game.start_movement(
            /*from=*/ &get_row_column(&game, String::from("f5")),
            /*to=*/ &get_row_column(&game, String::from("e6")),
            None,
        );

        assert_eq!(true, result.is_err());

        game.print_board()
    }

    #[test]
    fn simple_promotion_test() {
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let white_pawn_start = get_row_column(&game, String::from("a7"));
        let black_pawn_start = get_row_column(&game, String::from("a2"));
        let black_pawn = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::Black, true));
        let white_pawn = PieceType::Pawn.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&white_pawn_start, white_pawn);
        game.set_piece_row_col(&black_pawn_start, black_pawn);
        game.print_board();

        let result = game.handle_input(&String::from("move a7 a8"));
        assert_eq!(true, result.is_err());
        assert_eq!("We need to know what piece you want", result.err().unwrap());

        let result = game.handle_input(&String::from("move a7 a8Q"));
        assert_eq!(true, result.is_ok());
        game.print_board();

        // same for black
        let result = game.handle_input(&String::from("move a2 a1"));
        assert_eq!(true, result.is_err());
        assert_eq!("We need to know what piece you want", result.err().unwrap());

        let result = game.handle_input(&String::from("move a2 a1R"));
        assert_eq!(true, result.is_ok());
        game.print_board();
    }

    #[test]
    fn knight_movement_testing() {
        let board = Board::create_empty_board();
        let mut game = Game::inject_board(board);
        let from = get_row_column(&game, String::from("c5"));
        let to = get_row_column(&game, String::from("c7"));
        let white_knight_placement = get_row_column(&game, String::from("c5"));
        let white_king = PieceType::Knight.create_actual_piece(PieceInfo::new(Color::White, true));

        game.set_piece_row_col(&from, white_king);

        let result = game.start_movement(&from, &to, None);

        assert_eq!(true, result.is_err());
    }
}
