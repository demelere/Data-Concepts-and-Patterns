use algorithms_in_practice::algorithms::graphs::factor_graph::{
    CircularFactorGraph, Factor, NodeType, Transform2D,
};

fn main() {
    // Create a factor graph with limited size
    let mut graph = CircularFactorGraph::new(max_nodes: 10, max_factors: 15);

    println!("Simulating robot movement and landmark detection...\n");

    // Add initial robot pose
    let pose1 = NodeType::RobotPose(1000);
    graph.add_node(pose1.clone());
    println!("Added initial robot pose");

    // Add a landmark
    let landmark1 = NodeType::Landmark("L1".to_string());
    graph.add_node(landmark1.clone());
    println!("Added landmark L1");

    // Add factor between robot and landmark
    let factor1 = Factor {
        source: pose1.clone(),
        target: landmark1.clone(),
        transform: Transform2D::new(2.0, 1.0, 0.1),
        uncertainty: 0.1,
        sensor_type: "LIDAR".to_string(),
        timestamp: 1000,
    };
    
    println!("\nAdding factor between pose1 and landmark1...");
    if let Some(cycle) = graph.add_factor(factor1) {
        println!("Found cycle! (unexpected at this point)");
    } else {
        println!("Factor added successfully");
    }

    // Add second robot pose
    let pose2 = NodeType::RobotPose(2000);
    graph.add_node(pose2.clone());
    println!("\nAdded second robot pose");

    // Add odometry factor
    let factor2 = Factor {
        source: pose1,
        target: pose2.clone(),
        transform: Transform2D::new(1.0, 0.5, 0.2),
        uncertainty: 0.05,
        sensor_type: "ODOMETRY".to_string(),
        timestamp: 2000,
    };
    
    println!("Adding odometry factor...");
    if let Some(cycle) = graph.add_factor(factor2) {
        println!("Found cycle! (unexpected at this point)");
    } else {
        println!("Factor added successfully");
    }

    // Add factor from second pose to landmark (creating a cycle)
    let factor3 = Factor {
        source: pose2,
        target: landmark1,
        transform: Transform2D::new(1.0, 0.5, -0.1),
        uncertainty: 0.1,
        sensor_type: "LIDAR".to_string(),
        timestamp: 2000,
    };
    
    println!("\nAdding factor that should create a cycle...");
    if let Some(cycle) = graph.add_factor(factor3) {
        println!("Cycle detected! Checking consistency...");
        let (error, is_consistent) = graph.check_cycle_consistency(&cycle);
        println!("Cycle error: {:.3}", error);
        println!("Cycle is consistent: {}", is_consistent);
    } else {
        println!("No cycle detected (unexpected!)");
    }

    println!("\nFinal graph state:");
    println!("Nodes: {}", graph.node_count());
    println!("Factors: {}", graph.factor_count());
}