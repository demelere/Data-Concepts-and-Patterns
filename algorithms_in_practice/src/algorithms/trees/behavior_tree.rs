use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum NodeStatus {
    Success,
    Failure,
    Running,
}

pub trait BehaviorNode {
    fn tick(&mut self, blackboard: &mut Blackboard) -> NodeStatus;
    fn reset(&mut self);
}

pub struct Blackboard {
    data: HashMap<String, f64>,
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&f64> {
        self.data.get(key)
    }
}

pub struct Sequence {
    children: Vec<Box<dyn BehaviorNode>>,
    current_child: usize,
}

impl Sequence {
    pub fn new(children: Vec<Box<dyn BehaviorNode>>) -> Self {
        Self {
            children,
            current_child: 0,
        }
    }
}

impl BehaviorNode for Sequence {
    fn tick(&mut self, blackboard: &mut Blackboard) -> NodeStatus {
        while self.current_child < self.children.len() {
            match self.children[self.current_child].tick(blackboard) {
                NodeStatus::Success => {
                    self.current_child += 1;
                }
                NodeStatus::Running => return NodeStatus::Running,
                NodeStatus::Failure => {
                    self.reset();
                    return NodeStatus::Failure;
                }
            }
        }
        self.reset();
        NodeStatus::Success
    }

    fn reset(&mut self) {
        self.current_child = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}

pub struct Selector {
    children: Vec<Box<dyn BehaviorNode>>,
    current_child: usize,
}

impl Selector {
    pub fn new(children: Vec<Box<dyn BehaviorNode>>) -> Self {
        Self {
            children,
            current_child: 0,
        }
    }
}

impl BehaviorNode for Selector {
    fn tick(&mut self, blackboard: &mut Blackboard) -> NodeStatus {
        while self.current_child < self.children.len() {
            match self.children[self.current_child].tick(blackboard) {
                NodeStatus::Success => {
                    self.reset();
                    return NodeStatus::Success;
                }
                NodeStatus::Running => return NodeStatus::Running,
                NodeStatus::Failure => {
                    self.current_child += 1;
                }
            }
        }
        self.reset();
        NodeStatus::Failure
    }

    fn reset(&mut self) {
        self.current_child = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}

// Example robot behavior nodes
pub struct CheckBatteryLevel;

impl BehaviorNode for CheckBatteryLevel {
    fn tick(&mut self, blackboard: &mut Blackboard) -> NodeStatus {
        if let Some(battery_level) = blackboard.get("battery_level") {
            if *battery_level > 20.0 {
                NodeStatus::Success
            } else {
                NodeStatus::Failure
            }
        } else {
            NodeStatus::Failure
        }
    }

    fn reset(&mut self) {}
}

pub struct MoveToTarget;

impl BehaviorNode for MoveToTarget {
    fn tick(&mut self, blackboard: &mut Blackboard) -> NodeStatus {
        if let (Some(current_x), Some(target_x)) = (blackboard.get("current_x"), blackboard.get("target_x")) {
            let distance = (target_x - current_x).abs();
            if distance < 0.1 {
                NodeStatus::Success
            } else {
                // Simulate movement
                let new_x = current_x + (target_x - current_x).signum() * 0.1;
                blackboard.set("current_x", new_x);
                NodeStatus::Running
            }
        } else {
            NodeStatus::Failure
        }
    }

    fn reset(&mut self) {}
}

pub struct PerformTask;

impl BehaviorNode for PerformTask {
    fn tick(&mut self, blackboard: &mut Blackboard) -> NodeStatus {
        if let Some(battery_level) = blackboard.get("battery_level") {
            // Simulate task execution using battery
            let new_level = *battery_level - 1.0;
            blackboard.set("battery_level", new_level);
            NodeStatus::Success
        } else {
            NodeStatus::Failure
        }
    }

    fn reset(&mut self) {}
}