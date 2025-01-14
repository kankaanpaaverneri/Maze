use generate_grid::{constants::Input, grid};
mod generate_grid;
use std::io::stdin;

fn main() {
    //let mut grid = generate_grid();

    let mut grid = grid::generate_grid_with_characters();
    grid::dig_correct_path_to_maze(&mut grid);

    loop {
        std::process::Command::new("clear").status().unwrap();
        //print_full_grid(&grid);
        grid::print_limited_view(&grid, 2);

        let command = read_user_input();
        if let Input::Exit = command {
            grid::print_full_grid(&grid);
            break;
        }
        let game_complete = grid::move_player(&mut grid, command);
        if game_complete {
            std::process::Command::new("clear").status().unwrap();
            grid::print_full_grid(&grid);
            println!("Game complete");
            break;
        }
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        }
    }
    return buffer;
}

fn print_error(error_message: &str) {
    println!("{error_message}");
    println!("Try again\n");
}

fn read_user_input() -> Input {
    return loop {
        let buffer = read_stdin();
        if buffer.len() == 1 {
            print_error("No characters in input");
            continue;
        };
        let input = buffer.trim().parse();
        match input {
            Ok(input) => {
                let is_valid = is_input_valid(input);
                if let Input::Invalid = is_valid {
                    print_error("Invalid character");
                    continue;
                }
                break is_valid;
            }
            Err(_) => {
                print_error("Too many characters in the string");
                continue;
            }
        }
    };
}

fn is_input_valid(input: char) -> Input {
    match input {
        'w' => Input::Up,
        'a' => Input::Left,
        's' => Input::Down,
        'd' => Input::Right,
        'e' => Input::Exit,
        _ => Input::Invalid,
    }
}