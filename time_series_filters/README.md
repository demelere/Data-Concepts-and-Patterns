## Data characteristics
1. If your data exhibits gradual and smooth transitions (e.g. economic data), [Exponential Smoothing](#exponential-smoothing-filter) or [Moving Average](#moving-average-filter) are effective.
2. If your data has abrupt changes (e.g. stock market prices), use filters that preserve edges like [Gaussian](#gaussian-filter) or [Savitzky-Golay](#savitzky-golay-filter) are more suitable.
3. If your data is very noisy (e.g. raw sensor data), ise a [Median](#median-filter) filter to remove noise without affecting the signal too much.  For moderate noise, [Gaussian](#gaussian-filter) or [Wavelet](#wavelet-filter) filters may also work.  
4. If your data has clear periodic patterns (e.g. seasonal patterns in sales data), use [Fourier Transform](#fourier-transform-filter).  [Wavelet](#wavelet-filter) transforms might do well with data with complex, non-repetitive patterns.
5. If your data changes its statistical properties over time (stationarity, e.g. tracking a moving object), [Kalman Filters](#kalman-filter) may be ideal.

## Problem types
1. If your problem involves predicting future values, [Kalman](#kalman-filter) and [Exponential Smoothing](#exponential-smoothing-filter) are often used.
2. If your problem involves smoothing out fluctuations and noise, [Moving Average](#moving-average-filter), [Gaussian](#gaussian-filter), or [Savitzky-Golay](#savitzky-golay-filter) are appropriate.
3. If the signal and noise are distinctly different (e.g. audio or brain signal processing), [Butterworth](#butterworth-filter) filters can be helpful.
4. If the signal and noise are intertwined, you may need [Wavelet](#wavelet-filter) transforms.
5. If your problem requires real-time processing, use [Kalman](#kalman-filter) filters
6. If your problem requires maintaining the integrity of sudden changes (e.g. image edges), [Savitzky-Golay](#savitzky-golay-filter) or [Median](#median-filter) filters may be the right choice.

## Filter types

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
