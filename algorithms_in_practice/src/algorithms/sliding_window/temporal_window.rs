use std::collections::VecDeque;

/// Configuration for temporal window processing
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub dilations: Vec<usize>,      // Multiple dilation rates for different temporal scales
    pub snippet_lengths: Vec<usize>, // Window sizes for each dilation
    pub strides: Vec<usize>,        // Stride for each dilation level
    pub cap_dilation: bool,         // Whether to limit dilation based on sequence length
}

/// Results from processing a temporal window
#[derive(Debug, Clone)]
pub struct WindowResult<T> {
    pub processed_data: Vec<T>,
    pub window_indices: Vec<Vec<usize>>,
    pub effective_dilations: Vec<usize>,
}

/// Processes temporal sequences using variable-dilation sliding windows
pub struct TemporalWindowProcessor {
    config: WindowConfig,
}

impl TemporalWindowProcessor {
    pub fn new(config: WindowConfig) -> Self {
        // Validate configuration
        assert!(!config.dilations.is_empty(), "Dilations cannot be empty");
        assert!(
            config.dilations.contains(&1),
            "Dilations must include rate of 1"
        );

        // Ensure consistent lengths or expand single values
        let n_scales = config.dilations.len();
        let snippet_lengths = if config.snippet_lengths.len() == 1 {
            vec![config.snippet_lengths[0]; n_scales]
        } else {
            assert_eq!(
                config.snippet_lengths.len(),
                n_scales,
                "Inconsistent number of snippet lengths"
            );
            config.snippet_lengths.clone()
        };

        let strides = if config.strides.len() == 1 {
            vec![config.strides[0]; n_scales]
        } else {
            assert_eq!(
                config.strides.len(),
                n_scales,
                "Inconsistent number of strides"
            );
            config.strides.clone()
        };

        Self {
            config: WindowConfig {
                dilations: config.dilations,
                snippet_lengths,
                strides,
                cap_dilation: config.cap_dilation,
            },
        }
    }

    /// Process a sequence of data using multiple dilation rates
    pub fn process_sequence<T, F>(
        &self,
        sequence: &[T],
        mut process_window: F,
    ) -> WindowResult<T> 
    where
        T: Clone,
        F: FnMut(&[T]) -> T,
    {
        let seq_len = sequence.len();
        let mut processed_data = Vec::new();
        let mut all_window_indices = Vec::new();
        let mut effective_dilations = Vec::new();

        // Process for each dilation rate
        for (scale_idx, &dilation) in self.config.dilations.iter().enumerate() {
            let snippet_len = self.config.snippet_lengths[scale_idx];
            let stride = self.config.strides[scale_idx];

            // Cap dilation if needed
            let effective_dilation = if self.config.cap_dilation {
                self.cap_max_dilation(seq_len, snippet_len, dilation)
            } else {
                dilation
            };
            effective_dilations.push(effective_dilation);

            // Get window indices for this dilation rate
            let window_indices = self.get_window_indices(
                seq_len,
                snippet_len,
                effective_dilation,
                stride,
            );
            all_window_indices.push(window_indices.clone());

            // Process each window
            let mut scale_results = VecDeque::new();
            for window_idx in window_indices {
                // Extract window data
                let window_data: Vec<T> = window_idx
                    .iter()
                    .map(|&idx| sequence[idx].clone())
                    .collect();

                // Process window
                let result = process_window(&window_data);
                scale_results.push_back(result);
            }

            // Store results for this scale
            processed_data.extend(scale_results);
        }

        WindowResult {
            processed_data,
            window_indices: all_window_indices,
            effective_dilations,
        }
    }

    /// Calculate valid window indices for a given dilation rate
    fn get_window_indices(
        &self,
        seq_len: usize,
        snippet_len: usize,
        dilation: usize,
        stride: usize,
    ) -> Vec<Vec<usize>> {
        let gap = dilation - 1;
        let total_window_size = (snippet_len - 1) * (gap + 1) + 1;
        
        // Calculate starting indices
        let mut start_indices = Vec::new();
        let mut current = 0;
        while current <= seq_len - total_window_size {
            start_indices.push(current);
            current += stride;
        }
        // Add final window if needed
        if start_indices.is_empty() || 
           *start_indices.last().unwrap() < seq_len - total_window_size {
            start_indices.push(seq_len - total_window_size);
        }

        // Generate window indices
        start_indices
            .into_iter()
            .map(|start| {
                (0..snippet_len)
                    .map(|i| start + i * (gap + 1))
                    .collect()
            })
            .collect()
    }

    /// Cap dilation rate based on sequence length
    fn cap_max_dilation(
        &self,
        seq_len: usize,
        snippet_len: usize,
        dilation: usize,
    ) -> usize {
        let max_allowed_gap = (seq_len / snippet_len) - 1;
        std::cmp::min(max_allowed_gap, dilation)
    }

    /// Verify that all frames are covered by at least one window
    pub fn verify_coverage(&self, seq_len: usize, window_indices: &[Vec<usize>]) -> bool {
        let mut covered = vec![false; seq_len];
        for window in window_indices {
            for &idx in window {
                covered[idx] = true;
            }
        }
        covered.iter().all(|&x| x)
    }
}