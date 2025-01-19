### Data Concepts and Patterns

Notebook of mathematical, physical, and other theoretical and dense underpinnings relevant to AI, robotics, and spatial computing.  The goal is to implement widely used robotics algorithms or mathematical proofs in Python.

#### Algorithms in Practice

##### Dynamic Programming
###### Robotics

##### Graphs
###### Robotics
The example demonstrates a simple pathfinding algorithm using a grid-based approach. It uses a breadth-first search (BFS) algorithm to find the shortest path from a start position to a goal position on a grid with obstacles.

Overview
The example demonstrates a simple pathfinding algorithm using a grid-based approach. It uses a breadth-first search (BFS) algorithm to find the shortest path from a start position to a goal position on a grid with obstacles.
Flow of Execution
1. Grid Initialization:
Input: A 10x10 grid is created using Grid::new(10, 10).
Output: A Grid object with specified dimensions and no obstacles initially.

2. Obstacle Setup:
Input: Obstacles are added to the grid to create a maze-like environment.
Output: The grid now contains obstacles at specified positions, making some paths impassable.

Path Planner Initialization:
Input: A RobotPathPlanner is initialized with the grid.
Output: A RobotPathPlanner object that can be used to find paths on the grid.

Define Start and Goal Positions:
Input: Start position (1, 1) and goal position (8, 8) are defined using Position::new.
Output: Two Position objects representing the start and goal.

Pathfinding Execution:
Input: The find_path method is called on the RobotPathPlanner with the start and goal positions.
Output:
If a path is found, it returns a vector of Position objects representing the path.
If no path is found, it returns None.

Path Reconstruction:
Input: If a path is found, the reconstruct_path method is used to backtrack from the goal to the start using the came_from map.
Output: A vector of Position objects representing the path from start to goal.

7. Path Length Calculation:
Input: The path vector is passed to calculate_path_length.
Output: The total length of the path in grid units, calculated using the Manhattan distance between consecutive positions.

Grid Visualization:
Input: The path vector is passed to print_path.
Output: A visual representation of the grid is printed, showing the path (*), obstacles (█), and empty spaces (·).
Output:
The program prints the path length and the coordinates of each step in the path.

Graph algorithms are used to find the shortest path from a start position to a goal position on a grid. The grid is treated as a graph where each cell is a node, and edges exist between adjacent cells. Here's how graph algorithms are applied in this context:

Graph Representation
1. Nodes:
Each cell in the grid is considered a node in the graph. The Position struct represents these nodes with x and y coordinates.
2. Edges:
Edges exist between adjacent cells (nodes) that are not blocked by obstacles. This is akin to a 4-connected grid where each node can connect to its north, south, east, and west neighbors.
Graph Algorithm: Breadth-First Search (BFS)
The pathfinding algorithm used in this example is a form of Breadth-First Search (BFS), which is a classic graph traversal algorithm. Here's how BFS is applied:
1. Initialization:
A queue (VecDeque) is initialized with the start position.
A visited set is used to keep track of nodes that have already been explored.
A came_from map is used to reconstruct the path once the goal is reached.
2. Traversal:
The algorithm dequeues a node (current position) from the front of the queue.
It checks if the current node is the goal. If so, it reconstructs the path using the came_from map.
For each valid neighbor of the current node (using get_neighbors), if the neighbor hasn't been visited, it is marked as visited, added to the queue, and its parent is recorded in the came_from map.

Path Reconstruction:
Once the goal is reached, the path is reconstructed by backtracking from the goal to the start using the came_from map.

4. Output:
If a path is found, it is returned as a sequence of Position objects. If no path is found, the function returns None.

##### Sliding Window
###### Robotics

###### Computer Vision
This is a video depth estimation pipeline that converts regular RGB video frames into depth maps (showing how far each pixel is from the camera). It adapts a latent diffusion model (LDM) that was originally designed for single images to work with videos, while maintaining temporal consistency (making sure the depth estimates don't flicker or jump between frames).

The main trick here is using a sliding window approach that looks at small groups of frames (typically 3 frames at a time, called "snippets") rather than trying to process the entire video at once. This makes the problem more tractable while still maintaining temporal consistency.

###### Processing Flow:
```Takes input RGB video frames
Encodes them into a latent space using a VAE (Variational Autoencoder)
Uses a UNet to predict depth in this latent space
Decodes the results back to depth maps
Includes a refinement step to improve temporal consistency```

###### Sliding Window Approach:
```Processes frames in small groups (snippets)
Uses variable "dilations" (frame gaps) to capture both local and long-range temporal relationships
Starts with larger dilations for global consistency, then reduces them for local refinement```

##### Behavior Trees
###### Robotics
Modules and Imports:
The `behavior_tree.rs` file in the examples directory uses the behavior tree components defined in `src/algorithms/trees/behavior_tree.rs`.  The `mod.rs` file in `src/algorithms/trees` re-exports the behavior tree components, making them accessible to the example.  The `Cargo.toml` file specifies the behavior_tree example, which is the entry point for running the behavior tree logic.

Initialization:
* Input: The `main` function initializes a Sequence node with two children: `CheckBatteryLevel` and a `Selector` node.
* Output: A behavior tree structure is created, ready to be executed.

2. Blackboard Setup:
* Input: The `Blackboard` is initialized with initial values for `battery_level`, `current_x`, and `target_x`.
* Output: The Blackboard holds the state of the robot, which will be used by the behavior nodes.

Behavior Tree Execution:
* Input: The `tick` method is called on the `robot_behavior` (a `Sequence` node) for 20 iterations.
* Output: Each tick updates the state of the Blackboard and prints the status of the task.

Node Execution:
* CheckBatteryLevel:
  * Input: Reads battery_level from the Blackboard.
  * Output: Returns NodeStatus::Success if the battery level is above 20, otherwise NodeStatus::Failure.
* Selector Node:
  * Input: Executes its children (MoveToTarget and PerformTask) based on their status.
  * Output: Returns NodeStatus::Success if any child succeeds, NodeStatus::Running if a child is running, or NodeStatus::Failure if all children fail.
* MoveToTarget:
  * Input: Reads current_x and target_x from the Blackboard.
  * Output: Moves towards the target and updates current_x. Returns NodeStatus::Running until the target is reached, then NodeStatus::Success.
* PerformTask:
  * Input: Reads and updates battery_level in the Blackboard.
  * Output: Simulates task execution by reducing the battery level and returns NodeStatus::Success.

Output: Each tick prints the current battery level, position, and task status (Success, Running, or Failure).

As an example:
```
Tick 1:
CheckBatteryLevel succeeds (battery is 100).
MoveToTarget starts moving (position 0.0 to 0.1).
Output: "Task in progress..."
Tick 2-50:
MoveToTarget continues moving until the target is reached.
Once the target is reached, PerformTask executes, reducing the battery.
Output: "Task completed successfully!" once the task is done.
```

In this example, the concept of a behavior tree is used, which is a type of tree algorithm commonly employed in AI for decision-making processes, particularly in robotics and game development. Here's how tree algorithms are applied in this context:

**Structure of the Behavior Tree**
1. Nodes:
The behavior tree is composed of different types of nodes, each representing a specific behavior or decision point. In this example, the nodes are:
Sequence: Executes its children in order until one fails.
Selector: Executes its children in order until one succeeds.
Leaf Nodes: These are the actual tasks or checks, such as CheckBatteryLevel, MoveToTarget, and PerformTask.

2. Tree Hierarchy:
The behavior tree is structured hierarchically, with the Sequence node at the root, containing a CheckBatteryLevel node and a Selector node as its children.
The Selector node further contains MoveToTarget and PerformTask as its children.

**Execution Flow**
1. Traversal:
The behavior tree is traversed from the root node down to the leaf nodes. This traversal is akin to a depth-first search (DFS) in tree algorithms, where each node is visited and its children are processed based on the node type (Sequence or Selector).
2. Decision Making:
Sequence Node: This node type requires all its children to succeed for it to return success. It processes each child in order, stopping at the first failure. This is similar to a logical AND operation.
Selector Node: This node type returns success if any of its children succeed. It processes each child in order, stopping at the first success. This is similar to a logical OR operation.
3. State Management:
The Blackboard acts as a shared memory space, allowing nodes to read and write state information. This is crucial for maintaining context across different parts of the tree.

**Tree Algorithm Characteristics**
Modularity: Each node encapsulates a specific behavior, making it easy to modify or extend the tree by adding or removing nodes.
Reactivity: The tree can react to changes in the environment (e.g., battery level) by adjusting the execution path based on node statuses.
Reusability: Nodes can be reused across different trees or scenarios, promoting code reuse.
Conclusion
While the behavior tree in this example is not a traditional data structure like a binary tree or AVL tree, it leverages tree-like properties to organize and execute decision-making logic. The hierarchical structure and traversal mechanisms are key aspects of how tree algorithms are applied in this context.

Sequence
│
├── CheckBatteryLevel
│
└── Selector
    │
    ├── MoveToTarget
    │
    └── PerformTask

Sequence Node:
This is the root node of the tree. It will execute its children in order and will only succeed if all its children succeed. If any child fails, it stops and returns failure.

CheckBatteryLevel:
This is the first child of the Sequence node. It checks if the battery level is above a certain threshold. If it succeeds, the Sequence node proceeds to the next child.

Selector Node:
This node is executed if CheckBatteryLevel succeeds. It will execute its children in order and will succeed if any of its children succeed. If all children fail, it returns failure.

MoveToTarget:
This is the first child of the Selector node. It attempts to move the robot towards a target position. It returns Running until the target is reached, then returns Success.

PerformTask:
This is the second child of the Selector node. It simulates a task that consumes battery power. It returns Success after execution.

This tree structure allows the robot to first check its battery level, and if sufficient, attempt to move to a target. If moving to the target fails, it will then attempt to perform a task. The behavior tree's modularity and hierarchical structure make it a powerful tool for organizing complex decision-making processes.



#### Geometry of Motion 
* Special orthogonal group SO(3) for rotations, and the special Euclidean group SE(3) for rigid body motions

#### Properties of rotating mass


#### Motion Planning and Pathfinding (Search)
Exploring the configuration space to generate feasible collision-free paths from start to goal in a given environment. They balance between exploration and exploitation to efficiently find optimal or near-optimal paths.
  
* A* Algorithm
* Rapidly-Exploring Random Trees (RRT)
* Probabilistic Roadmaps (PRM)
* Dijkstra's Algorithm


#### Localization and Mapping, and SLAM
Estimating the pose (position and orientation) of a robot within its environment (Localization) and building a map of the environment (Mapping).

* Kalman Filters
* Particle Filters
* Occupancy Grid Mapping
* Iterative Closest Point Algorithm

#### Kinematics and Dynamics

* Forward
* Inverse
* Lagrange
* Denavit-Hartenberg

* Lagrange multipliers and generalized coordinates (e.g. in a robotic arm with joints constrained to move in certain ways, the equations of motion can be derived using Lagrangian mechanics with constraints applied as Lagrange multipliers, ensuring movements adhere to mechanical limits).  

#### Spatial Representation and Transformation:
Representing and manipulating spatial orientations and transformations of objects or robots in three-dimensional space. They are fundamental for tasks like robot motion planning, kinematics, and control.

* 3D Rotation and Lie Groups: In robotics, understanding 3D rotations is crucial for representing the orientation of objects or robots in space. Lie groups are mathematical structures used to represent continuous groups of transformations, such as rotations and translations, in a compact and efficient manner. They are particularly useful for tasks like robot motion planning, kinematics, and control.  
  
* 3D Transformation: 3D transformations involve moving and orienting objects in three-dimensional space. This concept is essential for robotics to describe how objects move and interact with each other. Applications include robot arm kinematics, object manipulation, and 3D mapping/navigation for autonomous vehicles or drones.

* Rigid Body Transformation of Velocity: Rigid body transformations describe how velocities (linear and angular) are transformed between different coordinate frames attached to rigid bodies. This concept is essential for robot dynamics, motion control, and state estimation, allowing robots to relate velocities observed in one frame to velocities in another frame, enabling tasks like robot localization, mapping, and obstacle avoidance.

#### Optimization and Estimation:
Optimizing parameters and estimating states from sensor measurements. They are used in various robotics applications such as sensor fusion, simultaneous localization and mapping (SLAM), visual odometry, and camera calibration.

* Newton Methods for Optimization and Least-Squares Problems: Newton methods are iterative optimization algorithms used to find the minimum (or maximum) of a function. In robotics, they are applied to solve various optimization problems, such as robot trajectory planning, parameter estimation, or inverse kinematics. Least-squares problems involve minimizing the sum of the squares of the differences between observed and predicted values and are commonly used in data fitting, calibration, and localization tasks in robotics.

* Gauss-Newton Methods for Least-Squares Problems: Gauss-Newton methods are a variant of Newton's method specifically designed for solving nonlinear least-squares problems. They are widely used in robotics for applications like sensor calibration, simultaneous localization and mapping (SLAM), bundle adjustment in structure-from-motion, and camera pose estimation.

* Graph Optimization: Graph optimization techniques, such as the popular bundle adjustment and pose graph optimization, are used in robotics for refining the estimates of robot poses and landmarks based on noisy sensor measurements. These methods play a vital role in SLAM, visual odometry, simultaneous localization and mapping, and other tasks requiring accurate spatial reconstruction and mapping.

* Pre-Integration for Navigation: Pre-integration techniques are used in visual-inertial navigation systems (VINS) to combine measurements from both visual and inertial sensors. By integrating sensor data over time, pre-integration can provide more accurate estimates of a robot's pose and velocity, making it useful for applications like UAV navigation, indoor localization, and augmented reality.

* Camera Reprojection Error: Camera reprojection error measures the discrepancy between the observed image features and their corresponding projected positions in the camera's image plane. It is a common metric used in camera calibration, structure-from-motion, and visual SLAM systems to refine camera parameters and optimize the reconstruction accuracy.

To Do: 
* Diffusion
* Sensor fusion algorithms by abstraction level
* State space math involving Gaussian processes and Markov Chains
* Vector derivative transport theorem
* Quaternions for 3D and kinematics