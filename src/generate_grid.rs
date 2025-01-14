pub mod constants {
    pub const EMPTY: char = ' ';
    pub const WALL: char = '#';
    pub const PLAYER: char = 'p';
    pub const GRID_WIDTH: usize = 50;
    pub const GRID_HEIGHT: usize = 50;
    pub const WIN: char = 'W';
    pub const MOVES: char = '.';

    #[derive(Debug, PartialEq)]
    pub enum Input {
        Up,
        Down,
        Left,
        Right,
        Exit,
        Invalid,
    }
}

pub mod grid {

    use super::constants::*;

    pub struct Coordinates {
        column: usize,
        row: usize,
    }

    pub fn generate_grid_with_characters() -> [[char; GRID_WIDTH]; GRID_HEIGHT] {
        return [[WALL; GRID_WIDTH]; GRID_HEIGHT];
    }

    pub fn add_character_to_grid(
        grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT],
        character: char,
        coordinates: &Coordinates,
    ) {
        let Coordinates { column, row } = coordinates;
        grid[*column][*row] = character;
    }

    use rand::Rng;
    pub fn dig_correct_path_to_maze(grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT]) -> Coordinates {
        let dig_length = 500;
        let maze_hole = init_maze_hole();

        //Keep track of where we are on the grid while digging
        let mut current_position = Coordinates {
            column: maze_hole.column,
            row: maze_hole.row,
        };
        dig_initial_path(grid, &mut current_position);
        dig_correct_path(grid, &mut current_position, &dig_length);
        add_character_to_grid(grid, PLAYER, &current_position);
        let player_original_position = current_position;

        let mut empty_count = 0;
        let mut iteration_count = 0;
        while empty_count < dig_length {
            let mut random_position = get_random_position();
            dig_correct_path(grid, &mut random_position, &dig_length);
            empty_count = count_characters_in_grid(EMPTY, grid);
            iteration_count += 1;
        }
        println!("iteration_count = {}", iteration_count);
        add_character_to_grid(grid, WIN, &maze_hole);
        return player_original_position;
    }

    pub fn find_player(grid: &[[char; GRID_WIDTH]; GRID_HEIGHT]) -> Option<Coordinates> {
        for column in 0..GRID_HEIGHT {
            for row in 0..GRID_WIDTH {
                if grid[column][row] == PLAYER {
                    return Some(Coordinates { column, row });
                }
            }
            println!();
        }
        return None;
    }

    pub fn move_player(grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT], command: Input) -> bool {
        update_grid(grid, command)
    }
    pub fn print_full_grid_with_move_history(
        grid: &[[char; GRID_WIDTH]; GRID_HEIGHT],
        list_of_moves: &Vec<Coordinates>,
    ) {
        for column in 0..GRID_HEIGHT {
            for row in 0..GRID_WIDTH {
                if is_movement_in_list(list_of_moves, &Coordinates { column, row }) {
                    if grid[column][row] == EMPTY {
                        print!("{} ", MOVES);
                    } else {
                        print!("{} ", grid[column][row]);
                    }
                } else {
                    print!("{} ", grid[column][row]);
                }
            }
            println!();
        }
    }

    pub fn print_limited_view(grid: &[[char; GRID_WIDTH]; GRID_HEIGHT], view_distance: isize) {
        if let Some(coordinates) = find_player(grid) {
            let player_column = coordinates.column as isize;
            let player_row = coordinates.row as isize;

            for column in 0..GRID_HEIGHT as isize {
                for row in 0..GRID_WIDTH as isize {
                    let column_in_view_distance =
                        in_view_distance(column, player_column, view_distance);
                    let row_in_view_distance = in_view_distance(row, player_row, view_distance);

                    if column_in_view_distance && row_in_view_distance {
                        print!("{} ", grid[column as usize][row as usize]);
                    }
                }
                println!();
            }
        }
    }
    fn is_movement_in_list(list_of_moves: &Vec<Coordinates>, current_move: &Coordinates) -> bool {
        let Coordinates { column, row } = current_move;
        for movement in list_of_moves {
            if movement.column == *column && *row == movement.row {
                return true;
            }
        }
        return false;
    }

    fn count_characters_in_grid(
        character: char,
        grid: &[[char; GRID_WIDTH]; GRID_HEIGHT],
    ) -> usize {
        let mut counter = 0;
        for column in 0..GRID_HEIGHT {
            for row in 0..GRID_WIDTH {
                if grid[column][row] == character {
                    counter += 1;
                }
            }
        }
        return counter;
    }

    fn dig_initial_path(
        grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT],
        current_position: &mut Coordinates,
    ) {
        grid[current_position.column][current_position.row] = EMPTY;
        let direction = get_start_direction(&current_position);
        for _ in 1..25 {
            dig_path(&direction, current_position, grid, true);
        }
    }

    fn dig_correct_path(
        grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT],
        current_position: &mut Coordinates,
        dig_length: &usize,
    ) {
        let mut direction = get_random_direction();
        for i in 1..*dig_length {
            if i % 5 == 0 {
                direction = get_new_unique_direction(direction);
            }
            dig_path(&direction, current_position, grid, false);
        }
    }

    fn get_new_unique_direction(old_direction: Input) -> Input {
        loop {
            let new_direction = get_random_direction();
            if old_direction != new_direction {
                break new_direction;
            }
        }
    }

    fn get_random_direction() -> Input {
        let mut rand_generator = rand::thread_rng();
        let random_value = rand_generator.gen_range(0..4);
        match random_value {
            0 => Input::Up,
            1 => Input::Down,
            2 => Input::Left,
            3 => Input::Right,
            _ => Input::Invalid,
        }
    }

    fn get_start_direction(maze_hole: &Coordinates) -> Input {
        let Coordinates { column, row } = maze_hole;
        if *column == 0 {
            return Input::Down;
        }

        if *column == GRID_HEIGHT {
            return Input::Up;
        }

        if *row == 0 {
            return Input::Right;
        }

        if *row == GRID_WIDTH {
            return Input::Left;
        }
        return Input::Invalid;
    }

    fn init_maze_hole() -> Coordinates {
        let mut rand_generator = rand::thread_rng();
        let maze_hole_x = rand_generator.gen_range(2..(GRID_WIDTH - 2));
        let maze_hole_y = rand_generator.gen_range(2..(GRID_HEIGHT - 2));

        let switch = rand_generator.gen_bool(0.5);
        let mut maze_hole = Coordinates { column: 0, row: 0 };
        if switch == true {
            maze_hole.column = maze_hole_y;
            maze_hole.row = 0;
        } else {
            maze_hole.column = 0;
            maze_hole.row = maze_hole_x
        }
        return maze_hole;
    }

    fn get_random_position() -> Coordinates {
        let mut rand_generator = rand::thread_rng();
        return Coordinates {
            column: rand_generator.gen_range(10..GRID_WIDTH - 10),
            row: rand_generator.gen_range(10..GRID_WIDTH - 10),
        };
    }

    fn dig_path(
        direction: &Input,
        current_position: &mut Coordinates,
        grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT],
        ignore_bounds: bool,
    ) -> bool {
        let Coordinates { column, row } = current_position;

        match direction {
            Input::Up => {
                if *column > 2 && *column < GRID_HEIGHT - 2 || ignore_bounds {
                    if grid[*column - 1][*row] != PLAYER {
                        grid[*column - 1][*row] = EMPTY;
                    }
                    *column -= 1;
                }
            }
            Input::Down => {
                if *column < GRID_HEIGHT - 2 && *column > 2 || ignore_bounds {
                    if grid[*column + 1][*row] != PLAYER {
                        grid[*column + 1][*row] = EMPTY;
                    }

                    *column += 1;
                }
            }
            Input::Right => {
                if *row < GRID_WIDTH - 2 && *row > 2 || ignore_bounds {
                    if grid[*column][*row + 1] != PLAYER {
                        grid[*column][*row + 1] = EMPTY;
                    }
                    *row += 1;
                }
            }
            Input::Left => {
                if *row > 2 && *row < GRID_WIDTH - 2 || ignore_bounds {
                    if grid[*column][*row - 1] != PLAYER {
                        grid[*column][*row - 1] = EMPTY;
                    }
                    *row -= 1;
                }
            }
            _ => {}
        }
        return true;
    }

    fn in_view_distance(coordinate: isize, player_coordinate: isize, view_distance: isize) -> bool {
        coordinate >= player_coordinate - view_distance
            && coordinate <= player_coordinate + view_distance
    }

    fn update_grid(grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT], direction: Input) -> bool {
        for column in 0..GRID_HEIGHT {
            for row in 0..GRID_WIDTH {
                if grid[column][row] == PLAYER {
                    let win = move_to_direction(grid, direction, row, column);
                    return win;
                }
            }
        }
        return false;
    }

    fn move_to_direction(
        grid: &mut [[char; GRID_WIDTH]; GRID_HEIGHT],
        direction: Input,
        row: usize,
        column: usize,
    ) -> bool {
        let mut win = false;
        match direction {
            Input::Left => {
                if row > 0 && grid[column][row - 1] != WALL {
                    if game_complete(grid, column, row - 1) {
                        win = true;
                    }
                    grid[column][row] = EMPTY;
                    grid[column][row - 1] = PLAYER;
                }
            }
            Input::Right => {
                if row < GRID_WIDTH - 1 && grid[column][row + 1] != WALL {
                    if game_complete(grid, column, row + 1) {
                        win = true;
                    }
                    grid[column][row] = EMPTY;
                    grid[column][row + 1] = PLAYER;
                }
            }
            Input::Up => {
                if column > 0 && grid[column - 1][row] != WALL {
                    if game_complete(grid, column - 1, row) {
                        win = true;
                    }
                    grid[column][row] = EMPTY;
                    grid[column - 1][row] = PLAYER;
                }
            }
            Input::Down => {
                if column < GRID_HEIGHT - 1 && grid[column + 1][row] != WALL {
                    if game_complete(grid, column + 1, row) {
                        win = true;
                    }
                    grid[column][row] = EMPTY;
                    grid[column + 1][row] = PLAYER;
                }
            }
            _ => {}
        };
        return win;
    }

    fn game_complete(grid: &[[char; GRID_WIDTH]; GRID_HEIGHT], column: usize, row: usize) -> bool {
        if grid[column][row] == WIN {
            return true;
        }
        return false;
    }
}
