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
        terminal::{
            disable_raw_mode, enable_raw_mode, size, Clear, ClearType, ScrollDown, SetSize,
        },
    };

    pub fn init_raw_terminal(
        grid: &mut [[char; GRID_HEIGHT]; GRID_WIDTH],
        player_original_position: Coordinates,
        list_of_moves: &mut Vec<Coordinates>,
    ) -> std::io::Result<()> {
        enable_raw_mode()?;
        let (columns, rows) = size()?;
        execute!(std::io::stdout(), SetSize(200, 200), Clear(ClearType::All))?;
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
                        if !handle_event(
                            grid,
                            list_of_moves,
                            player_original_position,
                            is_game_completed,
                        ) {
                            break;
                        }
                    }
                    KeyCode::Down => {
                        let is_game_completed = move_player(grid, Input::Down);
                        if !handle_event(
                            grid,
                            list_of_moves,
                            player_original_position,
                            is_game_completed,
                        ) {
                            break;
                        }
                    }
                    KeyCode::Left => {
                        let is_game_completed = move_player(grid, Input::Left);
                        if !handle_event(
                            grid,
                            list_of_moves,
                            player_original_position,
                            is_game_completed,
                        ) {
                            break;
                        }
                    }
                    KeyCode::Right => {
                        let is_game_completed = move_player(grid, Input::Right);
                        if !handle_event(
                            grid,
                            list_of_moves,
                            player_original_position,
                            is_game_completed,
                        ) {
                            break;
                        }
                    }
                    KeyCode::Esc => {
                        add_character_to_grid(grid, START, player_original_position);
                        print_full_grid_with_move_history(grid, list_of_moves);
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
    fn handle_event(
        grid: &mut [[char; GRID_HEIGHT]; GRID_WIDTH],
        list_of_moves: &mut Vec<Coordinates>,
        player_original_position: &Coordinates,
        is_game_complete: bool,
    ) -> bool {
        if let Err(e) = execute!(std::io::stdout(), ScrollDown(5)) {
            print!("{e}\r\n");
            return false;
        }
        print_limited_view(grid, 2);
        if is_game_complete {
            add_character_to_grid(grid, START, player_original_position);
            print_full_grid_with_move_history(grid, list_of_moves);
            print!("Game complete\r\n");
            return false;
        }

        if let Some(coordinates) = find_player(grid) {
            list_of_moves.push(coordinates);
        }
        return true;
    }
}
