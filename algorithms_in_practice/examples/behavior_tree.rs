use algorithms_in_practice::algorithms::trees::{
    BehaviorNode, Blackboard, CheckBatteryLevel, MoveToTarget, PerformTask,
    Selector, Sequence, NodeStatus,
};

fn main() {
    // Create behavior tree for a simple robot task
    let mut robot_behavior = Sequence::new(vec![
        Box::new(CheckBatteryLevel),
        Box::new(Selector::new(vec![
            Box::new(MoveToTarget),
            Box::new(PerformTask),
        ])),
    ]);

    // Initialize robot state
    let mut blackboard = Blackboard::new();
    blackboard.set("battery_level", 100.0);
    blackboard.set("current_x", 0.0);
    blackboard.set("target_x", 5.0);

    // Run behavior tree for several ticks
    println!("Starting robot task execution...");
    for tick in 1..=20 {
        println!("\nTick {}", tick);
        println!("Battery: {:.1}%", blackboard.get("battery_level").unwrap());
        println!("Position: {:.1}", blackboard.get("current_x").unwrap());
        
        match robot_behavior.tick(&mut blackboard) {
            NodeStatus::Success => println!("Task completed successfully!"),
            NodeStatus::Running => println!("Task in progress..."),
            NodeStatus::Failure => println!("Task failed!"),
        }
    }
}