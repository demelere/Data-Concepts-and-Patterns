### Data Concepts and Patterns

Notebook of mathematical, physical, and other theoretical and dense underpinnings relevant to AI, robotics, and spatial computing.  The goal is to implement widely used robotics algorithms or mathematical proofs in Python.

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
* Sensor fusion algorithms by abstraction level
* Diffusion and state space math involving Gaussian processes and Markov Chains
* Vector derivative transport theorem
* Quaternions for 3D and kinematnics