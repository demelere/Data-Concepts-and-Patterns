## Data characteristics
1. If the data exhibits gradual and smooth transitions (e.g. economic data), [Exponential Smoothing](#exponential-smoothing-filter) or [Moving Average](#moving-average-filter) are effective.
2. If the data has abrupt changes (e.g. stock market prices), use filters that preserve edges like [Gaussian](#gaussian-filter) or [Savitzky-Golay](#savitzky-golay-filter) are more suitable.
3. If your data is very noisy (e.g. raw sensor data), ise a [Median](#median-filter) filter to remove noise without affecting the signal too much.  For moderate noise, [Gaussian](#gaussian-filter) or [Wavelet](#wavelet-filter) filters may also work.  
4. If the data has clear periodic patterns (e.g. seasonal patterns in sales data), use [Fourier Transform](#fourier-transform-filter).  [Wavelet](#wavelet-filter) transforms might do well with data with complex, non-repetitive patterns.

## Problem types
1. If the data changes its statistical properties over time (stationarity, e.g. tracking a moving object), [Kalman Filters](#kalman-filter) may be ideal.

### 2. If the problem involves predicting future values

### 3. If the problem involves smoothing out fluctuations and noise

### 4. If the signal and noise are distinctly different

### 5. If the signal and noise are intertwined

### 6. If you the problem requires real-time application

### 7. If the problem requires maintaining the integrity of sudden changes
 
#### Moving average filter
Uses a moving average of the data points to smooth out short-term fluctuations and highlight long-term trends Calculates the average of different subsets of the full data set. Simple moving average and weighted moving average are common types.

#### Exponential smoothing filter
Gives more weight to recent data points, making it more responsive to changes in the underlying signal. 

#### Gaussian filter
Uses a Gaussian function to smooth the signal by averaging the data points within a certain range. 

#### Wavelet filter
Uses wavelet decomposition to separate the signal into different frequency bands, allowing for the removal of specific frequency components. 

#### Median filter
Replaces each data point with the median value of the data points in a certain range, effectively removing outliers. 

#### Fourier transform filter
Uses the Fourier transform to convert the signal from the time domain to the frequency domain, allowing for the removal of specific frequency components.

#### Kalman filter
Uses a combination of mathematical techniques to estimate the state of a system based on a series of noisy measurements. It is commonly used for filtering data in the presence of random noise and model uncertainty. 

#### Butterworth filter
A type of low-pass filter that removes high-frequency noise while preserving the low-frequency signal. 

#### Savitzky-Golay filter
Uses a polynomial fitting method to smooth the signal while preserving the signal's shape and features. 
