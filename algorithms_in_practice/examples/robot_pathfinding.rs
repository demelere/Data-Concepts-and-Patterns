use algorithms_in_practice::algorithms::graphs::{Grid, Position, RobotPathPlanner};

fn main() {
    // Create a 10x10 grid
    let mut grid = Grid::new(10, 10);

    // Add some obstacles to create a maze-like environment
    for x in 3..7 {
        grid.add_obstacle(Position::new(x, 5));
    }
    for y in 2..5 {
        grid.add_obstacle(Position::new(3, y));
    }
    
    let planner = RobotPathPlanner::new(grid);
    
    // Define start and goal positions
    let start = Position::new(1, 1);
    let goal = Position::new(8, 8);

    println!("Finding path from ({}, {}) to ({}, {})...", 
             start.x, start.y, goal.x, goal.y);

    // Find and display the path
    match planner.find_path(start, goal) {
        Some(path) => {
            println!("\nPath found! Length: {} units", planner.calculate_path_length(&path));
            println!("\nGrid visualization (* = path, █ = obstacle, · = empty):");
            planner.grid().print_path(&path);
            
            println!("\nPath coordinates:");
            for (i, pos) in path.iter().enumerate() {
                println!("Step {}: ({}, {})", i, pos.x, pos.y);
            }
        }
        None => println!("No path found!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        let grid = Grid::new(5, 5);
        let planner = RobotPathPlanner::new(grid);
        
        let start = Position::new(0, 0);
        let goal = Position::new(4, 4);
        
        let path = planner.find_path(start, goal).unwrap();
        assert_eq!(planner.calculate_path_length(&path), 8);
    }

    #[test]
    fn test_blocked_path() {
        let mut grid = Grid::new(3, 3);
        // Block the middle row
        grid.add_obstacle(Position::new(0, 1));
        grid.add_obstacle(Position::new(1, 1));
        grid.add_obstacle(Position::new(2, 1));
        
        let planner = RobotPathPlanner::new(grid);
        
        let start = Position::new(0, 0);
        let goal = Position::new(0, 2);
        
        assert!(planner.find_path(start, goal).is_none());
    }
}