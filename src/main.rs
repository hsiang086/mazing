use std::env;

mod app;
mod game;
mod ui;
mod utils;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a map path is provided
    let map_path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    app::run(map_path);
}
