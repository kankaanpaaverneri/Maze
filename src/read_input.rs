pub mod user_input {

    use crate::generate_grid::{
        constants::*,
        grid::{
            add_character_to_grid, find_player, move_player, print_full_grid_with_move_history,
            print_limited_view, Coordinates,
        },
    };
    use crossterm::{
        event::{read, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize},
    };

    pub fn init_raw_terminal(
        grid: &mut [[char; GRID_HEIGHT]; GRID_WIDTH],
        player_original_position: Coordinates,
        list_of_moves: &mut Vec<Coordinates>,
    ) -> std::io::Result<()> {
        enable_raw_mode()?;
        let (columns, rows) = size()?;
        execute!(std::io::stdout(), SetSize(50, 50), Clear(ClearType::All))?;
        print!("Press arrow keys to start moving around\r\n");
        start_event_loop(grid, list_of_moves, &player_original_position)?;

        execute!(std::io::stdout(), SetSize(columns, rows))?;
        Ok(())
    }
    fn start_event_loop(
        grid: &mut [[char; GRID_HEIGHT]; GRID_WIDTH],
        list_of_moves: &mut Vec<Coordinates>,
        player_original_position: &Coordinates,
    ) -> std::io::Result<()> {
        loop {
            let event = read()?;
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Up => {
                        let is_game_completed = move_player(grid, Input::Up);
                        if !handle_event(grid, list_of_moves, is_game_completed) {
                            handle_game_end(grid, list_of_moves, player_original_position)?;
                            break;
                        }
                    }
                    KeyCode::Down => {
                        let is_game_completed = move_player(grid, Input::Down);
                        if !handle_event(grid, list_of_moves, is_game_completed) {
                            handle_game_end(grid, list_of_moves, player_original_position)?;
                            break;
                        }
                    }
                    KeyCode::Left => {
                        let is_game_completed = move_player(grid, Input::Left);
                        if !handle_event(grid, list_of_moves, is_game_completed) {
                            handle_game_end(grid, list_of_moves, player_original_position)?;
                            break;
                        }
                    }
                    KeyCode::Right => {
                        let is_game_completed = move_player(grid, Input::Right);
                        if !handle_event(grid, list_of_moves, is_game_completed) {
                            handle_game_end(grid, list_of_moves, player_original_position)?;
                            break;
                        }
                    }
                    KeyCode::Esc => {
                        handle_game_end(grid, list_of_moves, player_original_position)?;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        execute!(std::io::stdout(), Clear(ClearType::All))?;
        disable_raw_mode()?;
        Ok(())
    }

    fn handle_event(
        grid: &mut [[char; GRID_HEIGHT]; GRID_WIDTH],
        list_of_moves: &mut Vec<Coordinates>,
        is_game_complete: bool,
    ) -> bool {
        print_limited_view(grid, 2);
        if is_game_complete {
            return false;
        }

        if let Some(coordinates) = find_player(grid) {
            list_of_moves.push(coordinates);
        }
        return true;
    }

    fn handle_game_end(
        grid: &mut [[char; GRID_HEIGHT]; GRID_WIDTH],
        list_of_moves: &mut Vec<Coordinates>,
        player_original_position: &Coordinates,
    ) -> std::io::Result<()> {
        execute!(std::io::stdout(), SetSize(200, 200))?;
        add_character_to_grid(grid, START, player_original_position);
        print_full_grid_with_move_history(grid, list_of_moves);
        print!("Press ESC key to quit\r\n");
        loop {
            let end_event = read()?;

            match end_event {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }
}
