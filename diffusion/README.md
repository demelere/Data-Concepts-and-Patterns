### Core idea behind diffusion models

The basic idea is to start with data from our training set and then gradually add random noise to it, step by step, until it's completely random noise. Later, we try to reverse this process—starting from the noise, we gradually remove it to get back to the original data (or something close to it)

Diffusion models aim to generate samples from a set that is learned from training examples, which we will denote by $\mathcal{K}$. For example, if we want to generate images, $\mathcal{K} \subset \mathbb{R}^{c\times h \times w}$ is the set of pixel values that correspond to realistic images. Diffusion models also work for $\mathcal{K}$ corresponding to modalities other than images, such as audio, video, robot trajectories, and even in discrete domains such as text generation.

In a nutshell, diffusion models are trained by:
1. Sampling $x_0 \sim \mathcal{K}$, noise level $\sigma \sim [\sigma_\min,
\sigma_\max]$, noise $\epsilon \sim N(0, I)$
2. Generating noisy data $x_\sigma = x_0 + \sigma \epsilon$
3. Predicting $\epsilon$ (direction of noise) from by minimizing squared loss

This amounts to training a $\theta$-parameterized neural network $\epsilon_\theta(x, \sigma)$, by minimizing the loss function


#### Training set $\mathcal{K}$
Training set: $\mathcal{K}$ is the mathematical way of representing the collection of all possible data/training examples that the model could learn from.  

$\mathbb{R}^{c\times h \times w}$ describes the space of these images/training data
So for images, $\mathbb{R}$ stands for real numbers, while ${c\times h \times w}$ represents the dimensions of the images in terms of color channels (c), height (h), and width (w).

#### Adding noise
Original Example ($x_0$): This is your starting point, an original piece of data from the set $\mathcal{K}$.
