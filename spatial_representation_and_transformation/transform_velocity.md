# Rigid Body Transformation of Velocity

## Introduction
Consider two objects A and B moving in 3D space. The translational and angular velocities of A are known. Assuming that A and B are mounted on the same rigid body, we want to calculate the translational and angular velocities of B.

This type of problem is commonly encountered in the field of robotics. For example, when calculating the velocity of a LiDAR mounted on a vehicle given the velocity of the wheel centers obtained from odometry, or when calculating the velocity of the end effector given the joint velocities of a robot Arm.

However, deriving the solution to this problem requires knowledge of Lie groups and is not easy. This text uses the knowledge of Lie groups discussed in the first part to explain the answer and solution to this problem.

___