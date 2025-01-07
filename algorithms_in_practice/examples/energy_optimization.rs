use algorithms_in_practice::algorithms::dynamic_programming::{Task, MissionPlanner};

fn main() {
    // Define tasks for a long-duration robotic mission
    let tasks = vec![
        // Task(id, energy_cost, time_cost, priority, dependencies)
        Task::new(0, 20.0, 30, 3, vec![]),           // Initial system check
        Task::new(1, 50.0, 60, 5, vec![0]),          // Area mapping
        Task::new(2, 30.0, 45, 4, vec![0]),          // Sensor calibration
        Task::new(3, 80.0, 90, 8, vec![1, 2]),       // Sample collection
        Task::new(4, 40.0, 50, 6, vec![2]),          // Data transmission
        Task::new(5, 60.0, 75, 7, vec![3]),          // Sample analysis
        Task::new(6, 25.0, 40, 3, vec![4]),          // System maintenance
        Task::new(7, 35.0, 55, 5, vec![3, 4]),       // Environment monitoring
    ];

    // Create mission planner with constraints
    let max_energy = 300.0;  // Maximum battery capacity
    let max_time = 400;      // Mission time limit
    
    let mut planner = MissionPlanner::new(tasks, max_energy, max_time);

    println!("Planning optimal mission schedule...");
    println!("Maximum energy: {:.1} units", max_energy);
    println!("Maximum time: {} units", max_time);
    println!();

    // Optimize mission
    match planner.optimize_mission() {
        Some((value, sequence)) => {
            println!("Optimal mission plan found!");
            println!("Mission value: {:.2}", value);
            println!();
            
            println!("Task sequence:");
            for (i, &task_id) in sequence.iter().enumerate() {
                let task = &planner.tasks[task_id];
                println!(
                    "{}. Task {} (Energy: {:.1}, Time: {}, Priority: {})",
                    i + 1,
                    task_id,
                    task.energy_cost,
                    task.time_cost,
                    task.priority
                );
            }
            
            let total_energy = planner.calculate_energy_consumption(&sequence);
            let total_time = planner.calculate_total_time(&sequence);
            
            println!();
            println!("Total energy consumption: {:.1} units", total_energy);
            println!("Total time: {} units", total_time);
            println!("Energy efficiency: {:.2}%", (total_energy / max_energy) * 100.0);
            println!("Time efficiency: {:.2}%", (total_time as f64 / max_time as f64) * 100.0);
        }
        None => println!("No valid mission plan found! Check task dependencies and constraints."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mission() {
        let tasks = vec![
            Task::new(0, 10.0, 20, 2, vec![]),
            Task::new(1, 20.0, 30, 3, vec![0]),
        ];

        let mut planner = MissionPlanner::new(tasks, 50.0, 100);
        let result = planner.optimize_mission().unwrap();
        
        assert_eq!(result.1, vec![0, 1]);  // Both tasks should be scheduled
        assert!(result.0 > 0.0);  // Should have positive value
    }

    #[test]
    fn test_energy_constraint() {
        let tasks = vec![
            Task::new(0, 30.0, 20, 2, vec![]),
            Task::new(1, 80.0, 30, 3, vec![]),  // Exceeds energy limit
        ];

        let mut planner = MissionPlanner::new(tasks, 50.0, 100);
        let result = planner.optimize_mission().unwrap();
        
        assert_eq!(result.1, vec![0]);  // Only first task should be scheduled
    }

    #[test]
    fn test_cyclic_dependencies() {
        let tasks = vec![
            Task::new(0, 10.0, 20, 2, vec![1]),
            Task::new(1, 20.0, 30, 3, vec![0]),  // Creates cycle
        ];

        let mut planner = MissionPlanner::new(tasks, 50.0, 100);
        assert!(planner.optimize_mission().is_none());  // Should fail due to cycle
    }
}