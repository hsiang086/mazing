use crate::game::{Cell, Map, Player};
use crate::ui::draw_button;
use crate::utils::{get_maps_list, key_to_char, key_to_digit};
use raylib::prelude::*;
use std::collections::HashSet;
use std::fs::{self};
use std::path::Path;

use super::state::{GameState, NewGameState};

pub fn run(map_path: Option<String>) {
    // Initialize the window and graphics
    let (mut rl, thread) = raylib::init()
        .size(1000, 800)
        .title("Maze Game")
        .resizable()
        .build();

    // Set target FPS
    rl.set_target_fps(60);

    // Create maps directory if it doesn't exist
    if !Path::new("maps").exists() {
        fs::create_dir("maps").expect("Failed to create maps directory");
    }

    let mut state = GameState::MainMenu;
    let mut map_option = None;
    let mut player = Player::new(1, 0);

    let mut visited_positions = HashSet::new();
    let mut show_solution = false;

    // If a map path is provided, attempt to load it
    if let Some(path) = map_path {
        match Map::load_from_file(&path) {
            Ok(loaded_map) => {
                map_option = Some(loaded_map);
                state = GameState::Playing;
                visited_positions.insert(player.position);
            }
            Err(e) => {
                println!("Failed to load map from {}: {}", path, e);
                println!("Starting with Main Menu instead.");
                // Proceed to Main Menu
            }
        }
    }

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        // Get current window dimensions
        let window_width = rl.get_screen_width() as f32;
        let window_height = rl.get_screen_height() as f32;

        // UI dimensions
        let ui_width = 200.0;
        let maze_width = window_width - ui_width;

        match &mut state {
            GameState::MainMenu => {
                if let Some(new_state) = main_menu(
                    &mut rl,
                    &thread,
                    &mut map_option,
                    &mut visited_positions,
                    &mut show_solution,
                    window_width,
                    window_height,
                ) {
                    state = new_state;
                }
            }
            GameState::NewGameEnterWidth(ref mut new_game_state) => {
                if let Some(new_state) = new_game_enter_width(
                    &mut rl,
                    &thread,
                    new_game_state,
                    window_width,
                    window_height,
                ) {
                    state = new_state;
                }
            }
            GameState::NewGameEnterHeight(ref mut new_game_state) => {
                if let Some(new_state) = new_game_enter_height(
                    &mut rl,
                    &thread,
                    new_game_state,
                    window_width,
                    window_height,
                ) {
                    state = new_state;
                }
            }
            GameState::NewGameEnterName(ref mut new_game_state) => {
                if let Some(new_state) = new_game_enter_name(
                    &mut rl,
                    &thread,
                    new_game_state,
                    &mut map_option,
                    &mut player,
                    &mut visited_positions,
                    window_width,
                    window_height,
                ) {
                    state = new_state;
                }
            }
            GameState::LoadGame => {
                if let Some(new_state) = load_game(
                    &mut rl,
                    &thread,
                    &mut map_option,
                    &mut player,
                    &mut visited_positions,
                    &mut show_solution,
                    window_width,
                    window_height,
                ) {
                    state = new_state;
                }
            }
            GameState::Playing => {
                if let Some(new_state) = playing(
                    &mut rl,
                    &thread,
                    delta_time,
                    &mut map_option,
                    &mut player,
                    &mut visited_positions,
                    &mut show_solution,
                    maze_width,
                    ui_width,
                    window_width,
                    window_height,
                ) {
                    state = new_state;
                }
            }
        }
    }
}

// Function for Main Menu state
fn main_menu(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    map_option: &mut Option<Map>,
    visited_positions: &mut HashSet<(usize, usize)>,
    show_solution: &mut bool,
    window_width: f32,
    window_height: f32,
) -> Option<GameState> {
    let mouse_pos = rl.get_mouse_position();
    let mouse_left_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // Define button rectangles
    let button_width = 300.0;
    let button_height = 50.0;
    let button_x = (window_width - button_width) / 2.0;
    let new_game_button_rect = Rectangle::new(button_x, 300.0, button_width, button_height);
    let load_game_button_rect = Rectangle::new(button_x, 370.0, button_width, button_height);

    let new_game_hovered = new_game_button_rect.check_collision_point_rec(mouse_pos);
    let load_game_hovered = load_game_button_rect.check_collision_point_rec(mouse_pos);

    if mouse_left_pressed {
        if new_game_hovered {
            // Start new game setup by entering width
            let new_game_state = NewGameState {
                width_input: String::new(),
                height_input: String::new(),
                name_input: String::new(),
            };
            *map_option = None;
            visited_positions.clear();
            *show_solution = false;
            return Some(GameState::NewGameEnterWidth(new_game_state));
        } else if load_game_hovered {
            // Load game
            visited_positions.clear();
            *show_solution = false;
            return Some(GameState::LoadGame);
        }
    }

    // Draw main menu
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    // Draw title
    let title_text = "Maze Game";
    let title_font_size = 60;
    let title_width = d.measure_text(title_text, title_font_size) as f32;
    d.draw_text(
        title_text,
        ((window_width - title_width) / 2.0) as i32,
        150,
        title_font_size,
        Color::DARKBLUE,
    );

    // Draw New Game button
    draw_button(&mut d, &new_game_button_rect, "New Game", new_game_hovered);

    // Draw Load Game button
    draw_button(
        &mut d,
        &load_game_button_rect,
        "Load Game",
        load_game_hovered,
    );

    None
}

// Function for entering maze width
fn new_game_enter_width(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    new_game_state: &mut NewGameState,
    window_width: f32,
    window_height: f32,
) -> Option<GameState> {
    // Handle user input for maze width
    if let Some(key) = rl.get_key_pressed() {
        if let Some(c) = key_to_digit(key) {
            new_game_state.width_input.push(c);
        } else if key == KeyboardKey::KEY_BACKSPACE {
            new_game_state.width_input.pop();
        } else if key == KeyboardKey::KEY_ENTER {
            // Proceed to height input
            return Some(GameState::NewGameEnterHeight(new_game_state.clone()));
        }
    }

    // Cache values before drawing
    let mouse_pos = rl.get_mouse_position();
    let mouse_left_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // Back button
    let back_button_rect = Rectangle::new(20.0, 20.0, 100.0, 40.0);
    let back_button_hovered = back_button_rect.check_collision_point_rec(mouse_pos);
    let back_button_clicked = back_button_hovered && mouse_left_pressed;

    // Draw UI for maze width input
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    let prompt_text = "Enter maze width (odd number):";
    let prompt_font_size = 30;
    let prompt_width = d.measure_text(prompt_text, prompt_font_size) as f32;
    d.draw_text(
        prompt_text,
        ((window_width - prompt_width) / 2.0) as i32,
        250,
        prompt_font_size,
        Color::BLACK,
    );

    let input_rect = Rectangle::new((window_width - 400.0) / 2.0, 300.0, 400.0, 50.0);
    d.draw_rectangle_rec(input_rect, Color::LIGHTGRAY);
    d.draw_rectangle_lines_ex(input_rect, 2.0, Color::BLACK);

    let text_width = d.measure_text(&new_game_state.width_input, 30) as f32;
    d.draw_text(
        &new_game_state.width_input,
        ((window_width - text_width) / 2.0) as i32,
        310,
        30,
        Color::BLACK,
    );

    let instruction_text = "Press ENTER to proceed";
    let instruction_font_size = 20;
    let instruction_width = d.measure_text(instruction_text, instruction_font_size) as f32;
    d.draw_text(
        instruction_text,
        ((window_width - instruction_width) / 2.0) as i32,
        370,
        instruction_font_size,
        Color::DARKGRAY,
    );

    // Draw Back button
    draw_button(&mut d, &back_button_rect, "Back", back_button_hovered);

    // Handle Back button click
    if back_button_clicked {
        return Some(GameState::MainMenu);
    }

    None
}

// Function for entering maze height
fn new_game_enter_height(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    new_game_state: &mut NewGameState,
    window_width: f32,
    window_height: f32,
) -> Option<GameState> {
    // Handle user input for maze height
    if let Some(key) = rl.get_key_pressed() {
        if let Some(c) = key_to_digit(key) {
            new_game_state.height_input.push(c);
        } else if key == KeyboardKey::KEY_BACKSPACE {
            new_game_state.height_input.pop();
        } else if key == KeyboardKey::KEY_ENTER {
            // Proceed to map name input
            return Some(GameState::NewGameEnterName(new_game_state.clone()));
        }
    }

    // Cache values before drawing
    let mouse_pos = rl.get_mouse_position();
    let mouse_left_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // Back button
    let back_button_rect = Rectangle::new(20.0, 20.0, 100.0, 40.0);
    let back_button_hovered = back_button_rect.check_collision_point_rec(mouse_pos);
    let back_button_clicked = back_button_hovered && mouse_left_pressed;

    // Draw UI for maze height input
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    let prompt_text = "Enter maze height (odd number):";
    let prompt_font_size = 30;
    let prompt_width = d.measure_text(prompt_text, prompt_font_size) as f32;
    d.draw_text(
        prompt_text,
        ((window_width - prompt_width) / 2.0) as i32,
        250,
        prompt_font_size,
        Color::BLACK,
    );

    let input_rect = Rectangle::new((window_width - 400.0) / 2.0, 300.0, 400.0, 50.0);
    d.draw_rectangle_rec(input_rect, Color::LIGHTGRAY);
    d.draw_rectangle_lines_ex(input_rect, 2.0, Color::BLACK);

    let text_width = d.measure_text(&new_game_state.height_input, 30) as f32;
    d.draw_text(
        &new_game_state.height_input,
        ((window_width - text_width) / 2.0) as i32,
        310,
        30,
        Color::BLACK,
    );

    let instruction_text = "Press ENTER to proceed";
    let instruction_font_size = 20;
    let instruction_width = d.measure_text(instruction_text, instruction_font_size) as f32;
    d.draw_text(
        instruction_text,
        ((window_width - instruction_width) / 2.0) as i32,
        370,
        instruction_font_size,
        Color::DARKGRAY,
    );

    // Draw Back button
    draw_button(&mut d, &back_button_rect, "Back", back_button_hovered);

    // Handle Back button click
    if back_button_clicked {
        return Some(GameState::MainMenu);
    }

    None
}

// Function for entering map name and generating the maze
fn new_game_enter_name(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    new_game_state: &mut NewGameState,
    map_option: &mut Option<Map>,
    player: &mut Player,
    visited_positions: &mut HashSet<(usize, usize)>,
    window_width: f32,
    window_height: f32,
) -> Option<GameState> {
    // Generate the maze if it's not already generated
    if map_option.is_none() {
        if let (Ok(mut width), Ok(mut height)) = (
            new_game_state.width_input.parse::<usize>(),
            new_game_state.height_input.parse::<usize>(),
        ) {
            // Ensure dimensions are odd numbers
            if width % 2 == 0 {
                width += 1;
            }
            if height % 2 == 0 {
                height += 1;
            }

            // Validate dimensions (you can set limits as needed)
            if width < 5 || height < 5 {
                // Dimensions too small, reset inputs
                new_game_state.width_input.clear();
                new_game_state.height_input.clear();
                return Some(GameState::NewGameEnterWidth(new_game_state.clone()));
            }

            // Generate the map with specified dimensions
            let mut new_map = Map::new(width, height);
            new_map.generate_maze();
            *map_option = Some(new_map);
        } else {
            // Invalid input, return to width input
            new_game_state.width_input.clear();
            new_game_state.height_input.clear();
            return Some(GameState::NewGameEnterWidth(new_game_state.clone()));
        }
    }

    // Handle user input for map name
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        if new_game_state.name_input.is_empty() {
            // Assign default name
            let mut index = 1;
            loop {
                let filename = format!("maps/map{}.bin", index);
                if !Path::new(&filename).exists() {
                    new_game_state.name_input = format!("map{}", index);
                    break;
                }
                index += 1;
            }
        }

        let filename = format!("maps/{}.bin", new_game_state.name_input);
        map_option
            .as_ref()
            .unwrap()
            .save_to_file(&filename)
            .expect("Failed to save the map");
        // Reset player position
        *player = Player::new(1, 0);
        visited_positions.clear();
        visited_positions.insert(player.position);
        return Some(GameState::Playing);
    } else if let Some(key) = rl.get_key_pressed() {
        if let Some(c) = key_to_char(key) {
            new_game_state.name_input.push(c);
        } else if key == KeyboardKey::KEY_BACKSPACE {
            new_game_state.name_input.pop();
        }
    }

    // Cache values before drawing
    let mouse_pos = rl.get_mouse_position();
    let mouse_left_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // Back button
    let back_button_rect = Rectangle::new(20.0, 20.0, 100.0, 40.0);
    let back_button_hovered = back_button_rect.check_collision_point_rec(mouse_pos);
    let back_button_clicked = back_button_hovered && mouse_left_pressed;

    // Draw UI for map name input
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    let prompt_text = "Enter map name:";
    let prompt_font_size = 30;
    let prompt_width = d.measure_text(prompt_text, prompt_font_size) as f32;
    d.draw_text(
        prompt_text,
        ((window_width - prompt_width) / 2.0) as i32,
        250,
        prompt_font_size,
        Color::BLACK,
    );

    let input_rect = Rectangle::new((window_width - 400.0) / 2.0, 300.0, 400.0, 50.0);
    d.draw_rectangle_rec(input_rect, Color::LIGHTGRAY);
    d.draw_rectangle_lines_ex(input_rect, 2.0, Color::BLACK);

    let text_width = d.measure_text(&new_game_state.name_input, 30) as f32;
    d.draw_text(
        &new_game_state.name_input,
        ((window_width - text_width) / 2.0) as i32,
        310,
        30,
        Color::BLACK,
    );

    let instruction_text = "Press ENTER to save";
    let instruction_font_size = 20;
    let instruction_width = d.measure_text(instruction_text, instruction_font_size) as f32;
    d.draw_text(
        instruction_text,
        ((window_width - instruction_width) / 2.0) as i32,
        370,
        instruction_font_size,
        Color::DARKGRAY,
    );

    // Draw Back button
    draw_button(&mut d, &back_button_rect, "Back", back_button_hovered);

    // Handle Back button click
    if back_button_clicked {
        return Some(GameState::MainMenu);
    }

    None
}

fn load_game(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    map_option: &mut Option<Map>,
    player: &mut Player,
    visited_positions: &mut HashSet<(usize, usize)>,
    show_solution: &mut bool,
    window_width: f32,
    window_height: f32,
) -> Option<GameState> {
    // Get mouse input before drawing
    let mouse_pos = rl.get_mouse_position();
    let mouse_left_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // Begin drawing
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    let title_text = "Select a map to load:";
    let title_font_size = 30;
    let title_width = d.measure_text(title_text, title_font_size) as f32;
    d.draw_text(
        title_text,
        ((window_width - title_width) / 2.0) as i32,
        50,
        title_font_size,
        Color::BLACK,
    );

    // Check if maps are available
    let maps = get_maps_list();
    if maps.is_empty() {
        let info_text = "No saved maps available.";
        let info_font_size = 20;
        let info_width = d.measure_text(info_text, info_font_size) as f32;
        d.draw_text(
            info_text,
            ((window_width - info_width) / 2.0) as i32,
            100,
            info_font_size,
            Color::GRAY,
        );
    } else {
        // Handle input for map selection
        let start_y = 100.0;
        for (i, map_file) in maps.iter().enumerate() {
            let button_rect = Rectangle::new(
                (window_width - 400.0) / 2.0,
                start_y + i as f32 * 60.0,
                400.0,
                50.0,
            );
            let button_hovered = button_rect.check_collision_point_rec(mouse_pos);

            draw_button(&mut d, &button_rect, map_file, button_hovered);

            if button_hovered && mouse_left_pressed {
                // Load the selected map
                let map_path = format!("maps/{}", map_file);
                match Map::load_from_file(&map_path) {
                    Ok(loaded_map) => {
                        *map_option = Some(loaded_map);
                        // Reset player position
                        *player = Player::new(1, 0);
                        visited_positions.clear();
                        visited_positions.insert(player.position);
                        *show_solution = false;
                        return Some(GameState::Playing);
                    }
                    Err(e) => {
                        println!("Failed to load the map: {}", e);
                        // Optionally display an error message on the screen
                    }
                }
            }
        }
    }

    // Back button
    let back_button_rect = Rectangle::new(20.0, 20.0, 100.0, 40.0);
    let back_button_hovered = back_button_rect.check_collision_point_rec(mouse_pos);
    let back_button_clicked = back_button_hovered && mouse_left_pressed;

    // Draw Back button
    draw_button(&mut d, &back_button_rect, "Back", back_button_hovered);

    // Handle Back button click
    if back_button_clicked {
        return Some(GameState::MainMenu);
    }

    None
}

// Function for Playing state
fn playing(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    delta_time: f32,
    map_option: &mut Option<Map>,
    player: &mut Player,
    visited_positions: &mut HashSet<(usize, usize)>,
    show_solution: &mut bool,
    maze_width: f32,
    ui_width: f32,
    window_width: f32,
    window_height: f32,
) -> Option<GameState> {
    // Ensure map is loaded
    let map_ref = map_option.as_mut().unwrap();

    // Handle input
    if !player.is_moving {
        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
            if player.position.1 > 0 {
                let new_y = player.position.1 - 1;
                if let Some(Cell::Path) | Some(Cell::Solution) =
                    map_ref.get(player.position.0, new_y)
                {
                    player.direction = (0, -1);
                    player.is_moving = true;
                }
            }
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
            let new_y = player.position.1 + 1;
            if let Some(Cell::Path) | Some(Cell::Solution) = map_ref.get(player.position.0, new_y) {
                player.direction = (0, 1);
                player.is_moving = true;
            }
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            if player.position.0 > 0 {
                let new_x = player.position.0 - 1;
                if let Some(Cell::Path) | Some(Cell::Solution) =
                    map_ref.get(new_x, player.position.1)
                {
                    player.direction = (-1, 0);
                    player.is_moving = true;
                }
            }
        } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            let new_x = player.position.0 + 1;
            if let Some(Cell::Path) | Some(Cell::Solution) = map_ref.get(new_x, player.position.1) {
                player.direction = (1, 0);
                player.is_moving = true;
            }
        }
    }

    // Update player position
    player.update_position(delta_time);
    if !player.is_moving {
        visited_positions.insert(player.position);
    }

    // Cache values before drawing
    let mouse_pos = rl.get_mouse_position();
    let mouse_left_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    // Begin drawing
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    // Define maze and UI rectangles
    let maze_rect = Rectangle::new(0.0, 0.0, maze_width, window_height);
    let ui_rect = Rectangle::new(maze_width, 0.0, ui_width, window_height);

    // Calculate cell size
    let cell_size = maze_width / map_ref.width as f32;

    // Draw the maze
    for y in 0..map_ref.height {
        for x in 0..map_ref.width {
            let rect = Rectangle::new(
                maze_rect.x + x as f32 * cell_size,
                maze_rect.y + y as f32 * cell_size,
                cell_size,
                cell_size,
            );

            if map_ref.get(x, y) == Some(Cell::Wall) {
                d.draw_rectangle_rec(rect, Color::BLACK);
            } else if *show_solution && map_ref.get(x, y) == Some(Cell::Solution) {
                d.draw_rectangle_rec(rect, Color::YELLOW);
            } else if visited_positions.contains(&(x, y)) {
                d.draw_rectangle_rec(rect, Color::SKYBLUE); // Visited path
            } else {
                // Paths are left blank (white background)
            }
        }
    }

    // Draw the player
    let player_rect = Rectangle::new(
        maze_rect.x + player.render_position.0 * cell_size,
        maze_rect.y + player.render_position.1 * cell_size,
        cell_size,
        cell_size,
    );
    d.draw_rectangle_rec(player_rect, Color::RED);

    // Draw the exit
    let exit_rect = Rectangle::new(
        maze_rect.x + (map_ref.width - 2) as f32 * cell_size,
        maze_rect.y + (map_ref.height - 1) as f32 * cell_size,
        cell_size,
        cell_size,
    );
    d.draw_rectangle_rec(exit_rect, Color::GREEN);

    // Draw UI background
    d.draw_rectangle_rec(ui_rect, Color::LIGHTGRAY);

    // Draw UI elements within the window boundaries
    d.draw_text("Maze Game", (maze_width as i32) + 20, 20, 30, Color::BLACK);
    d.draw_text(
        "Use arrow keys or WASD to move.",
        (maze_width as i32) + 20,
        60,
        20,
        Color::BLACK,
    );

    // Display player position
    d.draw_text(
        &format!("Position: ({}, {})", player.position.0, player.position.1),
        (maze_width as i32) + 20,
        100,
        20,
        Color::BLACK,
    );

    // Optionally, display "You Win!" message
    if player.position.0 == map_ref.width - 2 && player.position.1 == map_ref.height - 1 {
        d.draw_text("You Win!", (maze_width as i32) + 50, 150, 30, Color::BLUE);
    }

    // Define button rectangles
    let button_width = ui_width - 40.0;
    let button_height = 40.0;
    let button_x = maze_width + 20.0;
    let solution_button_rect =
        Rectangle::new(button_x, window_height - 150.0, button_width, button_height);
    let menu_button_rect =
        Rectangle::new(button_x, window_height - 100.0, button_width, button_height);

    let solution_button_hovered = solution_button_rect.check_collision_point_rec(mouse_pos);
    let menu_button_hovered = menu_button_rect.check_collision_point_rec(mouse_pos);

    // Draw Show Solution button
    let solution_text = if *show_solution {
        "Hide Solution"
    } else {
        "Show Solution"
    };
    draw_button(
        &mut d,
        &solution_button_rect,
        solution_text,
        solution_button_hovered,
    );

    // Draw Menu button
    draw_button(&mut d, &menu_button_rect, "Menu", menu_button_hovered);

    // Handle button clicks
    if mouse_left_pressed {
        if solution_button_hovered {
            // Toggle show_solution
            *show_solution = !*show_solution;
            if *show_solution {
                map_ref.solve_maze();
            }
        } else if menu_button_hovered {
            // Go back to main menu
            *map_option = None;
            visited_positions.clear();
            *show_solution = false;
            return Some(GameState::MainMenu);
        }
    }

    None
}
