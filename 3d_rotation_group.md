# 3D rotation and Lie group

## Euler angles

Euler angles are a set of three angles introduced by Leonhard Euler that are used to describe the orientation or rotation of a rigid body in three-dimensional space.

$\alpha$: aka Roll, rotation around the x axis.  
$\beta$: aka Pitch, rotation around the y axis.  
$\gamma$: aka Yaw, rotation around the z axis.  

Rotation around the x axis.  
```math
R_x(\alpha) =  
\begin{bmatrix}  
1 & 0 & 0 \\  
0 & \cos \alpha & -\sin \alpha \\  
0 & \sin \alpha & \cos \alpha  
\end{bmatrix}  
```

Rotation around the y axis.
```math
R_y(\beta) =
\begin{bmatrix}
\cos \beta & 0 & \sin \beta \\
0 & 1 & 0 \\
-\sin \beta & 0 & \cos \beta
\end{bmatrix}
```

Rotation around the z axis.
```math
R_z(\gamma) = 
\begin{bmatrix}
\cos \gamma & -\sin \gamma & 0 & 0 \\
\sin \gamma & \cos \gamma & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
```

By combining these three angles, the orientation of the rigid body in three-dimensional space can be fully described.

### Rotation order of Euler angles

But since the multiplication of matrices does not satisfy the commutative law, different orders of x,y and z rotation generate different final rotation matrices.

```math
\begin{aligned} R_{xyz}
&=
R_z (\gamma) R_y (\beta) R_x (\alpha) \\
&=
\left[\begin{matrix} cos\gamma & -sin\gamma & 0 \\
sin\gamma & cos\gamma & 0 \ 0 & 0 & 1
\end{matrix}\right]
\left[\begin{matrix} cos\beta & 0 & sin\beta \\
0 & 1 & 0 \\
-sin\beta & 0 & cos\beta
\end{matrix}\right]
\left[\begin{matrix} 1 & 0 & 0 \\
0 & cos\alpha & -sin\alpha \\
0 & sin\alpha & cos\alpha
\end{matrix}\right] \\
&=
\left[\begin{matrix} c_{\gamma} c_{\beta} & c_{\gamma} s_{\beta} s_{\alpha} - c_{\alpha} s_{\gamma} & s_{\gamma} s_{\alpha} + c_{\gamma} c_{\alpha} s_{\beta} \\
c_{\beta} s_{\gamma} & c_{\gamma} c_{\alpha} + s_{\gamma} s_{\beta} s_{\alpha} & c_{\alpha} s_{\gamma} s_{\beta}- c_{\gamma} s_{\alpha} \\
-s_{\beta} & c_{\beta} s_{\alpha} & c_{\beta} c_{\alpha} \\
\end{matrix}\right]
\end{aligned} \tag{4}
```

```math
\begin{aligned} R_{zxy}
&=
R_y (\beta) R_x (\alpha) R_z (\gamma) \\
&=
\left[\begin{matrix} cos\beta & 0 & sin\beta \\
0 & 1 & 0 \\
-sin\beta & 0 & cos\beta
\end{matrix}\right]
\left[\begin{matrix} 1 & 0 & 0 \\
0 & cos\alpha & -sin\alpha \\
0 & sin\alpha & cos\alpha
\end{matrix}\right]
\left[\begin{matrix} cos\gamma & -sin\gamma & 0 \\
sin\gamma & cos\gamma & 0 \ 0 & 0 & 1
\end{matrix}\right] \\
&=
\left[\begin{matrix} c_{\beta} c_{\gamma}+s_{\beta} s_{\alpha} s_{\gamma} & c_{\gamma} s_{\beta} s_{\alpha}-c_{\beta} s_{\gamma} & c_{\alpha}s_{\beta} \\
c_{\alpha} s_{\gamma} & c_{\alpha} c_{\gamma} & -s_{\alpha} \\
c_{\beta} s_{\alpha} s_{\gamma}-s_{\beta} c_{\gamma} & s_{\beta} s_{\gamma}+c_{\beta} c_{\gamma} s_{\alpha} & c_{\beta} c_{\alpha}
\end{matrix}\right]
\end{aligned}
\tag{5}
```

Therefore, it is crucial to determine the rotation order when using Euler angles.

Euler angles are commonly used, however, they are not continuous due to the gimbal lock problem. Additionally, handling them in various mathematical problems can be challenging because of their non-commutative nature.

### Infinitesimal Rotation

If the angles are small enough, the following approximations hold true:

* $cos(a) \approx 1$
* $sin(a)\approx a$
* $sin(a)sin(b) \approx 0$

By substituting these approximations into equations (8) or (9), we can obtain the same results.
```math
\begin{aligned}
R_{xyz} \approx R_{zxy} &\approx
\left[\begin{matrix} 1 &  -  \gamma & \beta \\
\gamma & 1 & - \alpha \\
-\beta & \alpha & 1 \\
\end{matrix}\right] \\
&= I +
\left[\begin{matrix} 0 &  -  \gamma & \beta \\
\gamma & 0 & - \alpha \\
-\beta & \alpha & 0 \\
\end{matrix}\right] \\
&= I + \skew{\omega}
\end{aligned}
\qquad if \quad \alpha, \beta, \gamma \ll 1
\tag{6}
```

Here, $\skew{\omega}$ is a [skew-symmetric matrix](https://en.wikipedia.org/wiki/Skew-symmetric_matrix). A skew-symmetric matrix is a square matrix where the transpose of the matrix is equal to the negation of the matrix itself.

Now that we can obtain a commutative represention of a small rotation by a 3d vector. The remaining question is how to represent a larger rotation?

If we desire a larger rotation, we can simply divide the 3D vector 
 into n pieces and compose them as follows:

 ```math
 R(\omega) =
\underbrace{(I+\frac{\skew{\omega}}{n}) \times ...  (I+\frac{\skew{\omega}}{n})}_\text{n factors}
=(I+\frac{\skew{\omega}}{n})^n
\tag{7}
```

For real numbers, this series is very famous, shows a way to compute the exponential function. Similarly, we can extend the definition of exponential function to skew-symmetric matrix.

```math
R(\omega)
=(I+\frac{\skew{\omega}}{n})^n = e^{\skew{\omega}}
\tag{8}
```

The exponential sum formula is also applicable to skew-symmetric matrix. Now, the exponential map (8) or (9) can transform a 3d vector into a rotation matrix.

```math
R(\omega)
= e^{\skew{\omega}}
=\sum_{k=0}^\infty \frac{\skew{\omega}^k}{k!}
\tag{9}
```

Actually, some part of Lie group theories have been described in above. The 3D rotation space $R$ is called as _special orthogonal group_ $SO(3)$. The 3d vector $\omega$ is called the Lie algebra associated with SO(3) by the exponential map.