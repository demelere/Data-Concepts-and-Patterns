use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Position) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Get valid neighboring positions in a grid
    fn get_neighbors(&self, grid: &Grid) -> Vec<Position> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        directions
            .iter()
            .map(|(dx, dy)| Position::new(self.x + dx, self.y + dy))
            .filter(|pos| grid.is_valid_position(pos))
            .collect()
    }
}

#[derive(Debug)]
pub struct Grid {
    width: i32,
    height: i32,
    obstacles: HashSet<Position>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }

    pub fn add_obstacle(&mut self, pos: Position) {
        self.obstacles.insert(pos);
    }

    pub fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x >= 0 
        && pos.x < self.width 
        && pos.y >= 0 
        && pos.y < self.height 
        && !self.obstacles.contains(pos)
    }

    pub fn print_path(&self, path: &[Position]) {
        let path_set: HashSet<_> = path.iter().collect();
        
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                if self.obstacles.contains(&pos) {
                    print!("█ ");
                } else if path_set.contains(&&pos) {
                    print!("* ");
                } else {
                    print!("· ");
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub struct RobotPathPlanner {
    grid: Grid,
}

impl RobotPathPlanner {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    /// Find shortest path using BFS
    pub fn find_path(&self, start: Position, goal: Position) -> Option<Vec<Position>> {
        if !self.grid.is_valid_position(&start) || !self.grid.is_valid_position(&goal) {
            return None;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = HashMap::new();

        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some(current) = queue.pop_front() {
            if current == goal {
                return Some(self.reconstruct_path(&came_from, &start, &goal));
            }

            for next in current.get_neighbors(&self.grid) {
                if !visited.contains(&next) {
                    visited.insert(next.clone());
                    came_from.insert(next.clone(), current.clone());
                    queue.push_back(next);
                }
            }
        }

        None
    }

    fn reconstruct_path(
        &self,
        came_from: &HashMap<Position, Position>,
        start: &Position,
        goal: &Position,
    ) -> Vec<Position> {
        let mut path = vec![goal.clone()];
        let mut current = goal;

        while current != start {
            current = &came_from[current];
            path.push(current.clone());
        }

        path.reverse();
        path
    }

    /// Calculate path length in grid units
    pub fn calculate_path_length(&self, path: &[Position]) -> u32 {
        path.windows(2)
            .map(|positions| positions[0].manhattan_distance(&positions[1]))
            .sum()
    }

    /// Get the underlying grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }
}