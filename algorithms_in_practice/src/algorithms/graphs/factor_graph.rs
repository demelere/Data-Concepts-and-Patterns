use std::collections::{HashMap, HashSet, VecDeque};
use nalgebra as na;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform2D {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

impl Transform2D {
    pub fn new(x: f64, y: f64, theta: f64) -> Self {
        Self { x, y, theta }
    }

    /// Compose two transforms
    pub fn compose(&self, other: &Transform2D) -> Transform2D {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();
        
        Transform2D {
            x: self.x + other.x * cos_theta - other.y * sin_theta,
            y: self.y + other.x * sin_theta + other.y * cos_theta,
            theta: (self.theta + other.theta) % (2.0 * std::f64::consts::PI),
        }
    }

    /// Calculate the inverse transform
    pub fn inverse(&self) -> Transform2D {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();
        
        Transform2D {
            x: -self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta - self.y * cos_theta,
            theta: -self.theta,
        }
    }

    /// Calculate difference from identity transform
    pub fn error_from_identity(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.theta.sin().powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    RobotPose(u64),      // Timestamped robot position
    Landmark(String),     // Named landmark
}

#[derive(Debug)]
pub struct Factor {
    pub source: NodeType,
    pub target: NodeType,
    pub transform: Transform2D,
    pub uncertainty: f64,
    pub sensor_type: String,
    pub timestamp: u64,
}

pub struct CircularFactorGraph {
    nodes: VecDeque<NodeType>,
    factors: VecDeque<Factor>,
    max_nodes: usize,
    max_factors: usize,
    parent: HashMap<NodeType, NodeType>,  // For Union-Find
    rank: HashMap<NodeType, usize>,       // For Union-Find optimization
}

impl CircularFactorGraph {
    pub fn new(max_nodes: usize, max_factors: usize) -> Self {
        Self {
            nodes: VecDeque::new(),
            factors: VecDeque::new(),
            max_nodes,
            max_factors,
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    /// Add a new node to the graph
    pub fn add_node(&mut self, node: NodeType) {
        if self.nodes.len() >= self.max_nodes {
            // Remove oldest node and its associated factors
            if let Some(old_node) = self.nodes.pop_front() {
                self.factors.retain(|factor| 
                    factor.source != old_node && factor.target != old_node
                );
                self.parent.remove(&old_node);
                self.rank.remove(&old_node);
            }
        }
        
        self.nodes.push_back(node.clone());
        self.parent.insert(node.clone(), node.clone());
        self.rank.insert(node, 0);
    }

    /// Add a new factor (edge) to the graph
    pub fn add_factor(&mut self, factor: Factor) -> Option<Vec<Factor>> {
        // Ensure nodes exist
        if !self.nodes.contains(&factor.source) || !self.nodes.contains(&factor.target) {
            return None;
        }

        // Check for cycle before adding factor
        let cycle = self.would_create_cycle(&factor);
        
        // Add factor to graph
        if self.factors.len() >= self.max_factors {
            self.factors.pop_front();
        }
        self.factors.push_back(factor);

        cycle
    }

    /// Find the root node in the Union-Find structure
    fn find(&mut self, node: &NodeType) -> NodeType {
        let parent_node = self.parent.get(node).unwrap().clone();
        if parent_node == *node {
            return node.clone();
        }
        
        let root = self.find(&parent_node);
        self.parent.insert(node.clone(), root.clone());
        root
    }

    /// Union two sets in the Union-Find structure
    fn union(&mut self, node1: &NodeType, node2: &NodeType) {
        let root1 = self.find(node1);
        let root2 = self.find(node2);
        
        if root1 != root2 {
            let rank1 = *self.rank.get(&root1).unwrap();
            let rank2 = *self.rank.get(&root2).unwrap();
            
            if rank1 < rank2 {
                self.parent.insert(root1, root2);
            } else if rank1 > rank2 {
                self.parent.insert(root2, root1);
            } else {
                self.parent.insert(root2, root1);
                self.rank.insert(root1, rank1 + 1);
            }
        }
    }

    /// Check if adding a factor would create a cycle
    fn would_create_cycle(&mut self, new_factor: &Factor) -> Option<Vec<Factor>> {
        let root1 = self.find(&new_factor.source);
        let root2 = self.find(&new_factor.target);
        
        if root1 == root2 {
            // Found a cycle, find the factors in it
            if let Some(cycle_factors) = self.find_cycle_factors(&new_factor.source, &new_factor.target) {
                return Some(cycle_factors);
            }
        }
        
        // No cycle found, union the nodes
        self.union(&new_factor.source, &new_factor.target);
        None
    }

    /// Find factors that form a cycle
    fn find_cycle_factors(&self, start: &NodeType, end: &NodeType) -> Option<Vec<Factor>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        if self.find_path(start, end, &mut visited, &mut path) {
            return Some(path);
        }
        None
    }

    /// DFS to find path between nodes
    fn find_path(
        &self,
        current: &NodeType,
        target: &NodeType,
        visited: &mut HashSet<NodeType>,
        path: &mut Vec<Factor>
    ) -> bool {
        if current == target && !path.is_empty() {
            return true;
        }

        visited.insert(current.clone());

        for factor in &self.factors {
            if factor.source == *current && !visited.contains(&factor.target) {
                path.push(factor.clone());
                if self.find_path(&factor.target, target, visited, path) {
                    return true;
                }
                path.pop();
            } else if factor.target == *current && !visited.contains(&factor.source) {
                path.push(factor.clone());
                if self.find_path(&factor.source, target, visited, path) {
                    return true;
                }
                path.pop();
            }
        }

        visited.remove(current);
        false
    }

    /// Check consistency of a cycle of factors
    pub fn check_cycle_consistency(&self, cycle: &[Factor]) -> (f64, bool) {
        let mut combined_transform = Transform2D::new(0.0, 0.0, 0.0);
        
        for factor in cycle {
            combined_transform = combined_transform.compose(&factor.transform);
        }
        
        let error = combined_transform.error_from_identity();
        let is_consistent = error < 1.0; // Threshold can be adjusted
        
        (error, is_consistent)
    }

    /// Get current number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get current number of factors
    pub fn factor_count(&self) -> usize {
        self.factors.len()
    }
}