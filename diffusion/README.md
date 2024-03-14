### Core idea behind diffusion models

The basic idea is to start with data from our training set and then gradually add random noise to it, step by step, until it's completely random noise. Later, we try to reverse this process—starting from the noise, we gradually remove it to get back to the original data (or something close to it)

Diffusion models aim to generate samples from a set that is learned from training examples, which we will denote by $\mathcal{K}$. For example, if we want to generate images, $\mathcal{K} \subset \mathbb{R}^{c\times h \times w}$ is the set of pixel values that correspond to realistic images. Diffusion models also work for $\mathcal{K}$ corresponding to modalities other than images, such as audio, video, robot trajectories, and even in discrete domains such as text generation.

In a nutshell, diffusion models are trained by:
1. Sampling $x_0 \sim \mathcal{K}$, noise level $\sigma \sim [\sigma_\min,
\sigma_\max]$, noise $\epsilon \sim N(0, I)$
2. Generating noisy data $x_\sigma = x_0 + \sigma \epsilon$
3. Predicting $\epsilon$ (direction of noise) from by minimizing squared loss

This amounts to training a $\theta$-parameterized neural network $\epsilon_\theta(x, \sigma)$, by minimizing the loss function




#### Understanding the training set $\mathcal{K}$
Training set: $\mathcal{K}$ is the mathematical way of representing the collection of all possible data/training examples that the model could learn from.  

$\mathbb{R}^{c\times h \times w}$ describes the space of these images/training data
So for images, $\mathbb{R}$ stands for real numbers, while ${c\times h \times w}$ represents the dimensions of the images in terms of color channels (c), height (h), and width (w).

#### Training the model x0
Start with an example: We pick an original piece of data from our set (K).
Add Noise: We then add some noise to this data. The amount of noise is determined by a noise level () that we choose from a range between a minimum and maximum value. The actual noise () is random but follows a known pattern (normally distributed, which means it's the kind of randomness you see in a lot of natural processes, where most values are near the mean).
Create Noisy Data (): This gives us a new, noisier version of our original data () The formula just means "original data plus some noise.

##### Add noise
Original Example ($x_0$): This is your starting point, an original piece of data from the set $\mathcal{K}$.

Noise Level (): We select a level of noise to add to our original data. This level is a value between a minimum () and a maximum (). It determines how much noise we're going to introduce.

Noise (): The actual noise added to the data. It's randomly generated but follows a normal distribution, indicated by N(0,I). This means it's centered around 0 (its mean is 0) and I indicates it has a standard deviation that spreads out evenly in all directions (isotropically).

##### Create noisy data
Noisy Data ( ): The equation simply means that we take the original piece of data ( ) and add some noise to it. The noise is scaled by , meaning we adjust the intensity of the noise based on the noise level we chose.

#### Learning to Reverse the Noise
Predicting the Noise: The next step is teaching a computer model to guess the noise we added. By doing this, we're effectively teaching it how to reverse the process and remove the noise.

Training the Model: We do this through a process called training, where we adjust the parameters (θ) of our model to get better at predicting the noise (ϵ). The model, (x,σ), tries to predict the noise that was added to the original data, given the noisy data and the noise level (σ).

Minimizing Loss: The model's performance is measured using a loss function, which tells us how far off its predictions are from the actual noise. The goal of training is to minimize this loss, meaning we want our model to be as accurate as possible in predicting the added noise.

Predicting the Noise (): The model, denoted as, is tasked with guessing the noise that was added to the original data. represents the parameters or the internal settings of the model that can be adjusted through training. The goal is to make these guesses as accurate as possible.

Loss Function: The quality of the model's guesses is evaluated using a loss function, which measures the difference between the model's predicted noise and the actual noise added. The objective of training is to minimize this difference, making the model's predictions as close as possible to the true noise.