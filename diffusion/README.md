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

#### Training the model $x_0$
* Start with an example ($x_0$): We pick an original piece of data from our set ($\mathcal{K}$).
* Add Noise: We then add some noise to this data. The amount of noise is determined by a noise level ($\sigma$) that we choose from a range between a minimum and maximum value. The actual noise ($\epsilon$) is random but follows a known pattern (normally distributed, which means it's the kind of randomness you see in a lot of natural processes, where most values are near the mean).
* Create Noisy Data (x, \sigma): This gives us a new, noisier version of our original data () The formula just means "original data plus some noise.

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

In practice, this is done by the following simple training_loop that is the heart of training the diffusion model:
```
def training_loop(loader  : DataLoader,
                  model   : nn.Module,
                  schedule: Schedule,
                  epochs  : int = 10000):
    optimizer = torch.optim.Adam(model.parameters())
    for _ in range(epochs):
        for x0 in loader:
            optimizer.zero_grad()
            sigma, eps = generate_train_sample(x0, schedule)
            eps_hat = model(x0 + sigma * eps, sigma)
            loss = nn.MSELoss()(eps_hat, eps)
            optimizer.backward(loss)
            optimizer.step()
```
It receives several arguments: 
* `loader`: A DataLoader that batches your training data, allowing the model to learn from a subset of the data at a time, which is efficient and effective for training on large datasets.
* `model`: Your neural network model, an instance of nn.Module, which will learn to reverse the diffusion process.
* `schedule`: A schedule that determines the noise level (σ) for each training sample.
* `epochs`: The number of times the training loop will run through the entire dataset.

Inside the training loop:
* Optimizer Setup: It uses Adam optimizer, a popular choice for deep learning tasks, to adjust the model's parameters (θ) to minimize the loss function.
* Looping Through Data: It iterates through each epoch and then through each batch of original data points (x0) from the DataLoader.
* Generating Noise Samples: For each x0, it generates a noise level (σ) and a noise vector (ϵ) using the generate_train_sample function.
* Model Prediction: The model predicts the noise () added to the noisy version of 
* Loss Calculation: It calculates the mean squared error (MSE) loss between the predicted noise () and the actual noise (ϵ).
* Backpropagation and Optimization: The optimizer adjusts the model parameters to minimize this loss, improving the model's prediction accuracy over iterations.

The training loop iterates over batches of x0, then samples noise level sigma and noise vector eps using generate_train_sample:
```
def generate_train_sample(x0: torch.FloatTensor, schedule: Schedule):
    sigma = schedule.sample_batch(x0)
    eps = torch.randn_like(x0)
    return sigma, eps
```

This function is responsible for preparing the training samples for the diffusion model.  

It's arguments are 
* x0, the original data points
* `schedule`, The schedule from which the noise level (σ) is sampled.

Process:
It samples a noise level (σ) for each batch based on the schedule.
It generates a random noise vector (ϵ) with the same shape as x0, following a standard normal distribution (torch.randn_like(x0)).

Returns: The function returns both the noise level (σ) and the noise vector (ϵ), which are then used to generate noisy data and train the model.

##### Noise schedules

In practice, $\sigma$ is not sampled uniformly from the interval $[\sigma_\min,
\sigma_\max]$, instead this interval is discretized into N distinct values called a $\sigma$ schedule: $\{ \sigma_t \}_{t=1}^N$, and $\sigma$ is instead sampled uniformly from the N possible values of $\sigma_t$. We define the Schedule class that encapsulates the list of possible sigmas, and sample from this list during training.

```
class Schedule:
    def __init__(self, sigmas: torch.FloatTensor):
        self.sigmas = sigmas
    def __getitem__(self, i) -> torch.FloatTensor:
        return self.sigmas[i]
    def __len__(self) -> int:
        return len(self.sigmas)
    def sample_batch(self, x0:torch.FloatTensor) -> torch.FloatTensor:
        return self[torch.randint(len(self), (x0.shape[0],))].to(x0)
```

We'll use a log-linear schedule defined below:
```
class ScheduleLogLinear(Schedule):
    def __init__(self, N: int, sigma_min: float=0.02, sigma_max: float=10):
        super().__init__(torch.logspace(math.log10(sigma_min), math.log10(sigma_max), N))
```
But other commonly used schedules include ScheduleDDPM for pixel-space diffusion models and ScheduleLDM for latent diffusion models such as Stable Diffusion. The following plot compares these three schedules with default parameters.


This approach allows the model to learn the process of reversing the diffusion (or noise addition) process, enabling it to generate data similar to the original dataset after training.

Predicting the Noise (): The model, denoted as, is tasked with guessing the noise that was added to the original data. represents the parameters or the internal settings of the model that can be adjusted through training. The goal is to make these guesses as accurate as possible.

Loss Function: The quality of the model's guesses is evaluated using a loss function, which measures the difference between the model's predicted noise and the actual noise added. The objective of training is to minimize this difference, making the model's predictions as close as possible to the true noise.