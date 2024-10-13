use crate::game::cell::Cell;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Cell>,
}

impl Map {
    // Creates a new map filled with walls
    pub fn new(width: usize, height: usize) -> Self {
        // Ensure the width and height are odd numbers
        let width = if width % 2 == 1 { width } else { width - 1 };
        let height = if height % 2 == 1 { height } else { height - 1 };

        Self {
            width,
            height,
            grid: vec![Cell::Wall; width * height],
        }
    }

    // Access cell in the grid with boundary checking
    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.grid[y * self.width + x])
        } else {
            None
        }
    }

    // Set a cell in the grid
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = cell;
        }
    }

    // Generate a maze using recursive backtracking
    pub fn generate_maze(&mut self) {
        let mut rng = thread_rng();

        // Starting point
        let mut start_x = rng.gen_range(1..self.width).max(1);
        let mut start_y = rng.gen_range(1..self.height).max(1);
        start_x -= start_x % 2;
        start_y -= start_y % 2;

        // Mark starting point as path
        self.set(start_x, start_y, Cell::Path);
        let mut open_cells = vec![(start_x, start_y)];

        // Main loop for generating the maze
        while let Some((x, y)) = open_cells.pop() {
            let neighbors = self.get_neighbors(x, y);
            if let Some(&(nx, ny)) = neighbors.choose(&mut rng) {
                open_cells.push((x, y));
                self.set(nx, ny, Cell::Path);
                self.set((x + nx) / 2, (y + ny) / 2, Cell::Path);
                open_cells.push((nx, ny));
            }
        }

        // Create openings at the top and bottom
        self.set(1, 0, Cell::Path); // Entrance at the top
        self.set(self.width - 2, self.height - 1, Cell::Path); // Exit at the bottom
    }

    // Get unvisited neighbors for maze generation
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        let directions = [(-2isize, 0), (2, 0), (0, -2), (0, 2)];

        for &(dx, dy) in &directions {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);

            if nx < self.width && ny < self.height && self.get(nx, ny) == Some(Cell::Wall) {
                neighbors.push((nx, ny));
            }
        }

        neighbors
    }

    // Solve the maze using BFS and mark the solution path
    pub fn solve_maze(&mut self) {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.width * self.height];
        let mut came_from = vec![None; self.width * self.height];

        let start = (1, 0);
        let end = (self.width - 2, self.height - 1);

        queue.push_back(start);
        visited[start.1 * self.width + start.0] = true;

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == end {
                break;
            }

            for &(dx, dy) in &[(0isize, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);

                if nx < self.width
                    && ny < self.height
                    && !visited[ny * self.width + nx]
                    && self.get(nx, ny) != Some(Cell::Wall)
                {
                    queue.push_back((nx, ny));
                    visited[ny * self.width + nx] = true;
                    came_from[ny * self.width + nx] = Some((x, y));
                }
            }
        }

        // Reconstruct the path from end to start
        let mut current = end;
        while let Some(prev) = came_from[current.1 * self.width + current.0] {
            if current != end && current != start {
                self.set(current.0, current.1, Cell::Solution);
            }
            current = prev;
        }
    }

    // Save the map to a binary file
    pub fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    // Load a map from a binary file
    pub fn load_from_file(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let map = bincode::deserialize_from(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(map)
    }
}
