mod generate_grid;
mod read_input;

use generate_grid::grid;
use read_input::user_input::init_raw_terminal;

fn main() {
    let mut grid = grid::generate_grid_with_characters();
    let player_original_position = grid::dig_correct_path_to_maze(&mut grid);
    let mut list_of_moves: Vec<grid::Coordinates> = Vec::new();
    if let Err(e) = init_raw_terminal(&mut grid, player_original_position, &mut list_of_moves) {
        println!("Error occurred when initializing terminal: {}", e);
        std::process::exit(1);
    }
}
