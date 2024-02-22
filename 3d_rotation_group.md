# 3D rotation and Lie group

## Euler angles

Euler angles are a set of three angles introduced by Leonhard Euler that are used to describe the orientation or rotation of a rigid body in three-dimensional space.

$\alpha$: aka Roll, rotation around the x axis.  
$\beta$: aka Pitch, rotation around the y axis.  
$\gamma$: aka Yaw, rotation around the z axis.  

Rotation around the x axis.  
```math
R_x($\alpha$) =  
\begin{bmatrix}  
1 & 0 & 0 \\  
0 & $\cos$ $\alpha$ & -$\sin$ $\alpha$ \\  
0 & $\sin$ $\alpha$ & $\cos$ $\alpha$  
\end{bmatrix}  
```

Rotation around the y axis.
$$\[
R_y(\beta) = \left[ \begin{array}{ccc}
\cos \beta & 0 & \sin \beta \\
0 & 1 & 0 \\
-\sin \beta & 0 & \cos \beta
\end{array} \right]
\]$$

Rotation around the z axis.
$$
\[
R_z(\gamma) = \left[ \begin{array}{cccc}
\cos \gamma & -\sin \gamma & 0 & 0 \\
\sin \gamma & \cos \gamma & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1
\end{array} \right]
\]
$$

