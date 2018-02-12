

use super::parse_utils;


fn day19() {
    let all_lines : Vec<String> = parse_utils::read_line_by_line("inp19.txt");
    let col: usize = *&all_lines[0].find(|c:char| c == '|').unwrap();
    let mut current: Option<char> = Some('|');
    let mut next_pos: (usize, usize) = (0, col);
    let mut current_move = Move::Down;
    let mut iteration = 0;
    while current.is_some() {
        iteration +=1;
        let line:&String =  &all_lines[next_pos.0];
        let c = line.chars().nth(next_pos.1);
        current = c;
        let (row,col) = next_pos;
        //  next_pos =
        match current.unwrap() {
            '|' => {
                match current_move {
                    Move::Right => next_pos =(row, col + 1),
                    Move::Left => next_pos =(row, col - 1),
                    Move::Up => next_pos =(row - 1, col),
                    Move::Down => next_pos =(row + 1, col)
                }
            }
            '-' => {
                match current_move {
                    Move::Right => next_pos =(row, col + 1),
                    Move::Left => next_pos =(row, col - 1),
                    Move::Up => next_pos =(row - 1, col),
                    Move::Down => next_pos =(row + 1, col)
                }
            }
            '+' => {

                match current_move {
                    Move::Right => {
                        match all_lines.get(row - 1){
                            Some(v) => {
                                match  v.chars().nth(col) {
                                    Some(' ') => {},
                                    Some(_) => {
                                        current_move =  Move::Up;
                                        next_pos =(row - 1, col)
                                    },
                                    None => {}
                                }
                            },
                            None => {}
                        };



                        match all_lines.get(row + 1){
                            Some(v) => {
                                match  v.chars().nth(col) {
                                    Some(' ') => {},
                                    Some(_) => {
                                        current_move =  Move::Down;
                                        next_pos =(row + 1, col)
                                    },
                                    None => {}
                                }
                            }
                            None => {}
                        };

                    },
                    Move::Left => {
                        match all_lines.get(row - 1){
                            Some(v) => {
                                match  v.chars().nth(col) {
                                    Some(' ') => {},
                                    Some(_) => {
                                        current_move =  Move::Up;
                                        next_pos =(row - 1, col)
                                    },
                                    None => {}
                                }
                            }
                            None => {}
                        };
                        match all_lines.get(row + 1){
                            Some(v) => {
                                match  v.chars().nth(col) {
                                    Some(' ') => {},
                                    Some(_) => {
                                        current_move =  Move::Down;
                                        next_pos = (row + 1, col)
                                    },
                                    None => {}
                                }
                            }
                            None => {}
                        };
                    }
                    Move::Up => {
                        match all_lines[row].chars().nth(col - 1){
                            Some(c) => {
                                match c {
                                    ' ' => {},
                                    _ => {
                                        current_move =  Move::Left;
                                        next_pos =(row, col - 1)
                                    }
                                }
                            }
                            None => {}
                        };
                        match all_lines[row].chars().nth(col + 1){
                            Some(c) => {
                                match c {
                                    ' ' => {},
                                    _ => {
                                        current_move =  Move::Right;
                                        next_pos =(row, col + 1)
                                    }
                                }
                            }
                            None => {}
                        };

                    }
                    Move::Down => {
                        match all_lines[row].chars().nth(col - 1){
                            Some(c) => {
                                match c {
                                    ' ' => {},
                                    _ => {
                                        current_move = Move::Left;
                                        next_pos =(row, col - 1)
                                    }
                                }

                            }
                            None => {}
                        }
                        match all_lines[row].chars().nth(col + 1){
                            Some(c) => {
                                match c {
                                    ' ' => {},
                                    _ => {
                                        current_move =  Move::Right;
                                        next_pos =(row, col + 1)
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                };


            },
            ' '=> break,
            _ => {
                println!("Letter {}", current.unwrap());
                // println!("Pos {} cl: {}", next_pos.0, next_pos.1);

                match current_move {
                    Move::Up => {
                        next_pos =(row -1, col)
                    },
                    Move::Down => {
                        next_pos =(row +1, col)
                    },
                    Move::Left => {
                        next_pos =(row , col - 1)
                    },
                    _ => {
                        next_pos = (row, col +1)
                    }
                }
                // Extend vector
            }
        };
    }
    println!("Iterations {}", iteration);
}



#[derive(PartialEq)]
enum Move {
    Down,
    Up,
    Left,
    Right
}
