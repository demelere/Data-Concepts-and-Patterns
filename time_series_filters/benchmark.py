import numpy as np
import matplotlib.pyplot as plt
from scipy.ndimage.filters import uniform_filter1d

# Generate synthetic data with gradual and smooth changes
np.random.seed(0)
time = np.arange(0, 100, 0.5)
data = np.sin(time) + np.random.normal(0, 0.5, len(time))

# Exponential Smoothing function
def exponential_smoothing(series, alpha):
    result = [series[0]]  # first value is same as series
    for n in range(1, len(series)):
        result.append(alpha * series[n] + (1 - alpha) * result[n-1])
    return result

# Apply Exponential Smoothing
smoothed_exp = exponential_smoothing(data, 0.3)

# Apply Moving Average
window_size = 5
smoothed_ma = uniform_filter1d(data, size=window_size)

# Plot the results
plt.figure(figsize=(14, 6))

# Original data plot
plt.subplot(1, 2, 1)
plt.plot(time, data, label='Original Data')
plt.title('Original Data')
plt.xlabel('Time')
plt.ylabel('Value')
plt.legend()

# Smoothed data plot
plt.subplot(1, 2, 2)
plt.plot(time, data, label='Original Data', alpha=0.5)
plt.plot(time, smoothed_exp, label='Exponential Smoothing', color='red')
plt.plot(time, smoothed_ma, label='Moving Average', color='green')
plt.title('Data After Applying Filters')
plt.xlabel('Time')
plt.ylabel('Value')
plt.legend()

plt.tight_layout()

# Save the figure
plt.savefig('time_series_filters.png', dpi=300)

plt.show()
