Run `pip install -r requirements.txt` on first use
Run `pip freeze > requirements.txt` if additional packages are needed
`source venv/bin/activate`

## Data characteristics
1. If your data exhibits gradual and smooth transitions (e.g. economic data), [Exponential Smoothing](#exponential-smoothing-filter) or [Moving Average](#moving-average-filter) are effective.
2. If your data has abrupt changes (e.g. stock market prices), use filters that preserve edges like [Gaussian](#gaussian-filter) or [Savitzky-Golay](#savitzky-golay-filter) are more suitable.
3. If your data is very noisy (e.g. raw sensor data), ise a [Median](#median-filter) filter to remove noise without affecting the signal too much.  For moderate noise, [Gaussian](#gaussian-filter) or [Wavelet](#wavelet-filter) filters may also work.  
4. If your data has clear periodic patterns (e.g. seasonal patterns in sales data), use [Fourier Transform](#fourier-transform-filter).  [Wavelet](#wavelet-filter) transforms might do well with data with complex, non-repetitive patterns.
5. If your data changes its statistical properties over time (stationarity, e.g. tracking a moving object), [Kalman Filters](#kalman-filter) may be ideal.

## Problem types
1. If your problem involves predicting future values, [Kalman](#kalman-filter) and [Exponential Smoothing](#exponential-smoothing-filter) are often used.
2. If your problem involves smoothing out fluctuations and noise (e.g. spectroscopy, chromatography), [Moving Average](#moving-average-filter), [Gaussian](#gaussian-filter), or [Savitzky-Golay](#savitzky-golay-filter) are appropriate.
3. If the signal and noise are distinctly different (e.g. any sort of signal processing like audio, brain computer interface, spectral), [Butterworth](#butterworth-filter) filters can be helpful.
4. If the signal and noise are intertwined, you may need [Wavelet](#wavelet-filter) transforms.
5. If your problem requires real-time processing, use [Kalman](#kalman-filter) filters
6. If your problem requires maintaining the integrity of sudden changes (e.g. image edges), [Savitzky-Golay](#savitzky-golay-filter) or [Median](#median-filter) filters may be the right choice.

## Filter types

#### Moving average filter
Uses a moving average of the data points to smooth out short-term fluctuations and highlight long-term trends
* Calculates the average of different subsets of the full data set. Simple moving average and weighted moving average are common types.
* Smooths out short-term fluctuations, easy to understand and implement.
* Lags behind the trend, not effective in forecasting future values.
* Stock market analysis, economic trends
```
import pandas as pd
import numpy as np

df = pd.DataFrame(np.random.randn(10,1))
sma=df.rolling(3).mean()
```

#### Exponential smoothing filter
Gives more weight to recent data points, making it more responsive to changes in the underlying signal. Assigns exponentially decreasing weights over time.  
* Simple, effective for data with a trend, good for short-term forecasting.
* Not suitable for data with seasonality or higher-order structures.
* Stock prices and economic indicators
```
import pandas as pd
import numpy as np

df = pd.DataFrame(np.random.randn(10,1))
ewm_df = df.ewm(com=0.5).mean()
```

#### Gaussian filter
Uses a Gaussian function to smooth the signal by averaging the data points within a certain range. 
* Applies a Gaussian kernel to smooth data. It's a linear filter that reduces noise while maintaining edges.
* Good at preserving edges in images and time series.
* Can blur sharp edges in high-contrast signals.
* Image processing, signal noise reduction
```
from scipy.ndimage import gaussian_filter
import numpy as np

a = np.arange(50, step=2).reshape((5,5))
gf = gaussian_filter(a, sigma=1)
```

#### Wavelet filter
Uses wavelet decomposition to separate the signal into different frequency bands, allowing for the removal of specific frequency components. 
* Decomposes a signal into its constituent wavelets, which are scaled and shifted versions of a mother wavelet.
* Good for non-stationary signals, captures both frequency and location in time.
* More complex, requires understanding of wavelet selection.
* Image compression, denoising signals
```
import pywt

cA, cD = pywt.dwt([1,2,3,4], 'dbl')
```

#### Median filter
Replaces each data point with the median value of the data points in a certain range, effectively removing outliers. 
* Replaces each entry with the median of neighboring entries. It is a non-linear filter.
* Effective in removing 'salt and pepper' type noise, preserves edges.
* Not suitable for smoothing out high-frequency components.
* Digital image processing, removing outliers in data
```
from scipy.signal import medfilt

fit = medfilt([2, 6, 5, 4, 0, 3, 5, 7, 9, 2, 0, 1], 5)
```

#### Fourier transform filter
Uses the Fourier transform to convert the signal from the time domain to the frequency domain, allowing for the removal of specific frequency components.
* Converts time series data into the frequency domain using the Fourier transform. It decomposes a function into its constituent frequencies.
* Identifies periodicities, good for signal processing
* Assumes periodic signals, can be complex to interpret
* Signal processing, spectral analysis
```
from scipy.fft import fft, ifft
import numpy as np

x = np.array([1.0, 2.0, 1.0, -1.0, 1.5])
y = fft(x)
```

#### Kalman filter
Uses a combination of mathematical techniques to estimate the state of a system based on a series of noisy measurements. It is commonly used for filtering data in the presence of random noise and model uncertainty. 
* A recursive filter that estimates the state of a linear dynamic system from a series of noisy measurements.
* Works well in real-time, good for systems with uncertainty.
* Assumes linear dynamics and Gaussian noise, complex to implement.
* Navigation, mobility systems, econometrics
```
from pykalman import KalmanFilter
import numpy as np

kf = KalmanFilter(
    transition_matrices=[[1,1],[0,1]]
    observation_matrice[[0.1,0.5],[-0.3,0.0]]
)
measurements = np.asarray([[1,0],[0,0],[0,1]])
kf = kf.em(measurements, n_iter=5)

filtered_state_means, filtered_state_covariances = kf.filter(measurements)
```

#### Butterworth filter
A type of low-pass filter that removes high-frequency noise while preserving the low-frequency signal. 
* Designed to have a frequency response as flat as possible in the passband. It uses a maximally flat magnitude filter.
* Smooth response in the pass band, relatively simple design.
* Slow roll-off response
* Audio processing, embedded hardware and sensors
```
from scipy.signal import butter, freqs

b, a = butter(4, 100, 'low', analog=True)
w, h = freqs(b,a)
```

#### Savitzky-Golay filter
Uses a polynomial fitting method to smooth the signal while preserving the signal's shape and features. 
* Performs a local polynomial regression to determine the smoothed values. It preserves higher momentums better than a simple moving average.
* Preserves features of the distribution such as relative maxima, minima, and width.
* Less effective for very noisy data
* Spectroscopy, chromatography
```
from scipy.signal import savgol_filter
import numpy as np

x = np.array([2,2,5,2,1,0,1,4,9])
filtered = savgol_filter(x,5,2)
```