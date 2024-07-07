use chess::game::game::Game;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[test]
fn playing_against_myself_before_check_mate_logic() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/black_wins.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    let movement: Vec<&str> = line.split(" ").collect();
                    if movement.len() == 3 {
                        let mut sb = String::from("move ");
                        sb.push_str(movement[1]);
                        sb.push_str(" ");
                        sb.push_str(movement[2]);
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}

#[test]
fn draw() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/draw.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() == 4 {
                        let mut sb = String::from("move ");
                        let mut iter = line.chars();

                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        sb.push_str(" ");
                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}

// features promotion
#[test]
fn white_wins() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/white_wins.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() >= 4 {
                        let mut sb = String::from("move ");
                        let mut iter = line.chars();

                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        sb.push_str(" ");
                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        if line.len() == 5 {
                            sb.push(iter.next().unwrap());
                        }
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}

#[test]
fn white_wins_aug_30() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/white_wins_august_30.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() >= 4 {
                        let mut sb = String::from("move ");
                        let mut iter = line.chars();

                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        sb.push_str(" ");
                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        if line.len() == 5 {
                            sb.push(iter.next().unwrap());
                        }
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                        if sb == "move c6 b5" {
                            game.print_board();
                            print!("check here");
                        }
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}

#[test]
fn draw_aug_30() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/draw_august_30.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() >= 4 {
                        // if line == "g7g8Q" {
                        //     print!("check here");
                        // }
                        let mut sb = String::from("move ");
                        let mut iter = line.chars();

                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        sb.push_str(" ");
                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        if line.len() == 5 {
                            sb.push(iter.next().unwrap());
                        }
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}

#[test]
fn draw_sept_1() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/draw_sept_1.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() >= 4 {
                        let mut sb = String::from("move ");
                        let mut iter = line.chars();

                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        sb.push_str(" ");
                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        if line.len() == 5 {
                            sb.push(iter.next().unwrap());
                        }
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}

#[test]
fn white_wins_sept_1() {
    let mut game = Game::new();
    if let Ok(lines) = read_lines("./tests/resources/white_wing_sept_1.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() >= 4 {
                        let mut sb = String::from("move ");
                        let mut iter = line.chars();

                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        sb.push_str(" ");
                        sb.push(iter.next().unwrap());
                        sb.push(iter.next().unwrap());
                        if line.len() == 5 {
                            sb.push(iter.next().unwrap());
                        }
                        let result = game.handle_input(&sb);
                        assert_eq!(true, result.is_ok(), "failed at {}", sb);
                    }
                }
                Err(error) => {
                    panic!("{}", error)
                }
            }
        }
    }
    game.print_board();
}
