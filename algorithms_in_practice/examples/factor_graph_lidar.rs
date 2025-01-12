use algorithms_in_practice::algorithms::graphs::factor_graph::{
    CircularFactorGraph, Factor, NodeType, Transform2D,
};
use std::{thread, time::Duration};
use rand::Rng;

struct LidarProcessor {
    current_timestamp: u64,
    processing_time_base: u64,  // Base processing time in ms
    scan_period: u64,           // Time between scans in ms
    is_faulty: bool,           // Simulated sensor fault
}

impl LidarProcessor {
    fn new(processing_time_base: u64, scan_period: u64) -> Self {
        Self {
            current_timestamp: 0,
            processing_time_base,
            scan_period,
            is_faulty: false,
        }
    }

    fn process_scan(&mut self) -> Option<Transform2D> {
        let mut rng = rand::thread_rng();
        
        // Simulate variable processing time
        let processing_time = self.processing_time_base as f64 * 
            (0.5 + rng.gen::<f64>());
        thread::sleep(Duration::from_millis(processing_time as u64));

        // Generate transform with some noise
        let base_noise = if self.is_faulty { 0.5 } else { 0.05 };
        Some(Transform2D::new(
            rng.gen::<f64>() * base_noise,
            rng.gen::<f64>() * base_noise,
            rng.gen::<f64>() * base_noise,
        ))
    }
}

fn simulate_odometry(timestamp: u64) -> Transform2D {
    let mut rng = rand::thread_rng();
    // Simulate roughly constant forward motion with small noise
    Transform2D::new(
        0.1 + rng.gen::<f64>() * 0.01,  // Mostly forward
        rng.gen::<f64>() * 0.01,         // Small lateral drift
        rng.gen::<f64>() * 0.01,         // Small rotation
    )
}

fn simulate_visual_landmark_detection(timestamp: u64, is_valid: bool) -> Option<(String, Transform2D)> {
    let mut rng = rand::thread_rng();
    
    if !is_valid || rng.gen::<f64>() < 0.3 {  // 30% chance of no detection
        return None;
    }

    // Generate a random landmark ID from a small set
    let landmark_id = format!("L{}", rng.gen_range(1..=3));
    
    // Generate transform to landmark with some noise
    let transform = Transform2D::new(
        rng.gen_range(-2.0..2.0),
        rng.gen_range(-2.0..2.0),
        rng.gen_range(-0.1..0.1),
    );

    Some((landmark_id, transform))
}

fn main() {
    let mut graph = CircularFactorGraph::new(100, 150);
    let mut lidar = LidarProcessor::new(40, 50);  // 40ms processing, 50ms between scans
    let mut rng = rand::thread_rng();

    println!("Starting robot localization simulation...");
    println!("- LIDAR scanning at 20Hz (50ms period)");
    println!("- Processing time varies around 40ms");
    println!("- Will introduce fault after 10 cycles\n");

    let mut inconsistent_cycles = 0;
    let mut total_cycles = 0;

    // Add initial robot pose
    let initial_pose = NodeType::RobotPose(0);
    graph.add_node(initial_pose.clone());

    // Run for 20 cycles
    for i in 0..20 {
        println!("\nCycle {}", i + 1);
        
        // Introduce fault after 10 cycles
        if i == 10 {
            println!("Introducing LIDAR fault...");
            lidar.is_faulty = true;
        }

        lidar.current_timestamp += lidar.scan_period;
        let current_pose = NodeType::RobotPose(lidar.current_timestamp);
        graph.add_node(current_pose.clone());

        // Add odometry factor
        let odom_transform = simulate_odometry(lidar.current_timestamp);
        let odom_factor = Factor {
            source: initial_pose.clone(),
            target: current_pose.clone(),
            transform: odom_transform,
            uncertainty: 0.1,
            sensor_type: "ODOMETRY".to_string(),
            timestamp: lidar.current_timestamp,
        };

        if let Some(cycle) = graph.add_factor(odom_factor) {
            println!("Found cycle from odometry!");
            let (error, is_consistent) = graph.check_cycle_consistency(&cycle);
            println!("  Error: {:.3}, Consistent: {}", error, is_consistent);
            total_cycles += 1;
            if !is_consistent {
                inconsistent_cycles += 1;
            }
        }

        // Process LIDAR scan
        if let Some(lidar_transform) = lidar.process_scan() {
            let lidar_factor = Factor {
                source: initial_pose.clone(),
                target: current_pose.clone(),
                transform: lidar_transform,
                uncertainty: 0.2,
                sensor_type: "LIDAR".to_string(),
                timestamp: lidar.current_timestamp,
            };

            if let Some(cycle) = graph.add_factor(lidar_factor) {
                println!("Found cycle from LIDAR!");
                let (error, is_consistent) = graph.check_cycle_consistency(&cycle);
                println!("  Error: {:.3}, Consistent: {}", error, is_consistent);
                total_cycles += 1;
                if !is_consistent {
                    inconsistent_cycles += 1;
                }
            }
        }

        // Try to detect visual landmarks
        if let Some((landmark_id, landmark_transform)) = 
            simulate_visual_landmark_detection(lidar.current_timestamp, !lidar.is_faulty) 
        {
            let landmark_node = NodeType::Landmark(landmark_id);
            graph.add_node(landmark_node.clone());

            let landmark_factor = Factor {
                source: current_pose.clone(),
                target: landmark_node,
                transform: landmark_transform,
                uncertainty: 0.15,
                sensor_type: "VISUAL".to_string(),
                timestamp: lidar.current_timestamp,
            };

            if let Some(cycle) = graph.add_factor(landmark_factor) {
                println!("Found cycle through landmark!");
                let (error, is_consistent) = graph.check_cycle_consistency(&cycle);
                println!("  Error: {:.3}, Consistent: {}", error, is_consistent);
                total_cycles += 1;
                if !is_consistent {
                    inconsistent_cycles += 1;
                }
            }
        }

        initial_pose = current_pose;
        
        println!("Graph size: {} nodes, {} factors", 
                graph.node_count(), graph.factor_count());
    }

    println!("\nSimulation complete!");
    println!("Total cycles found: {}", total_cycles);
    println!("Inconsistent cycles: {}", inconsistent_cycles);
    println!("Consistency rate: {:.1}%", 
             100.0 * (total_cycles - inconsistent_cycles) as f64 / total_cycles as f64);
    println!("LIDAR status: {}", 
             if inconsistent_cycles > total_cycles / 3 {
                 "Likely faulty"
             } else {
                 "Likely working correctly"
             });
}