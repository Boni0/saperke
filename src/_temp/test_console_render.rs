use crate::grid::{GridCellVariant, GridCellState};
use crate::game::{Game, GameState, GameEndState};

use std::{io};

fn test_render(game: &Game) {
    // Clean
    print!("\x1B[2J\x1B[1;1H");

    match &game.state {
       GameState::EndState(end_state) => {
            let end_state_str = match end_state {
                GameEndState::Loss => "You Lose!",
                GameEndState::Win => "You Win!"
            };

            println!(" {} ", end_state_str);
            return
       },
       _ => {}
    }

    let line_height = game.grid.width;
    let print_horizontal_line = || {
        print!("----");
        for idx in 1..=line_height {
            let hyphen = if idx <= 9 { "-" } else {""};
            print!("{}{}-", hyphen, idx);
        }
        print!("----\n");
    };
    
    print_horizontal_line();

    let mut line_idx = 0;
    for grid_line_vec in &game.grid.cells {
        let line_space = if line_idx <= 8 { " " } else { "" };
        print!("{}{} |", line_idx + 1, line_space);
        line_idx += 1;

        for grid_cell in grid_line_vec {
            let print_value = match grid_cell.variant {
                GridCellVariant::NonExist => String::from("X"),
                _ => {
                    match grid_cell.state {
                        GridCellState::Hidden => String::from("#"),
                        GridCellState::Tagged => String::from("!"),
                        GridCellState::Visible => {
                            match grid_cell.variant {
                                GridCellVariant::WithValue(value) => if value == 0 {
                                    String::from(" ")
                                } else { value.to_string() },
                                GridCellVariant::WithBomb => String::from("B"),
                                _ => String::from("#")
                            }
                        },
                        GridCellState::Questioned | _ => String::from("?"),
                    }
                }
            };

            print!(" {} ", print_value);
        }

        println!("|");
    }

    print_horizontal_line();
}

fn test_input_cord() -> (usize, usize) {
    let mut input_string = String::new();
    let mut y_cord: usize;
    let mut x_cord: usize;

    loop {
        println!("Please provide cell cordinates in comma-separated format y-cord x-cord (e.g \"2 3\")"); 

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                let mut cord_iter = input_string.split_whitespace();

                if let Some(y_cord_input) = cord_iter.next() {
                    match y_cord_input.to_string().parse::<usize>() {
                        Ok(value) => { 
                            y_cord = value; 
                            
                            if let Some(x_cord_input) = cord_iter.next() {
                                match x_cord_input.to_string().parse::<usize>() {
                                    Ok(value) => { x_cord = value; break; },
                                    Err(_) => { println!("Something gone wrong, please try again"); },
                                }
                            }
                        },
                        Err(_) => { println!("Something gone wrong, please try again"); },
                    }
                }
            },
            Err(_) => {
                println!("Something gone wrong, please try again");
            },
        }
    }

    (y_cord, x_cord)
}

fn test_input_type() -> String {
    let mut input_string = String::new();

    loop {
        println!("Please provide type of action for cell (T: toggle flag, V: view)"); 

        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                match input_string.trim() {
                    "T" | "V" => { break; }
                    _ => { println!("Something gone wrong, please try again: {}", input_string.as_str()); }
                }
            },
            Err(_) => {
                println!("Something gone wrong, please try again");
            },
        }
    }

    input_string
}

pub fn init_game_console() {
    let mut game = Game::new();

    loop {
        test_render(&game);

        if let GameState::EndState(_) = game.state { break; }

        let (y,x) = loop {
            let (y_input, x_input) = test_input_cord();
            if let Some(_) = game.grid.get_cell(y_input - 1, x_input - 1) {
                break (y_input - 1, x_input - 1)
            }
        };

        match test_input_type().trim() {
            "V" => {
                if let Some(variant) = game.grid.set_cell_visible(y, x) {
                    match variant {
                        GridCellVariant::WithValue(_) => {
                            if game.grid.cells_count == (game.grid.visible_count + game.mines_count) {
                                game.state = GameState::EndState(GameEndState::Win);
                            }
                        },
                        GridCellVariant::WithBomb => {
                            game.state = GameState::EndState(GameEndState::Loss);
                        },
                        _ => {},
                    }
                }
            },
            "T" => {
                game.grid.toggle_cell_tagged_state(y, x);
            },
            _ => {
                println!("HEH");
            }
        }

        
    }
}
