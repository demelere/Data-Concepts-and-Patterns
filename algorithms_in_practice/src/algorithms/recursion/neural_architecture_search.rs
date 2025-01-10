use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ActivationType {
    ReLU,
    Sigmoid,
    Tanh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayerType {
    Dense { units: usize },
    Convolution { filters: usize, kernel_size: usize },
    Pooling { pool_size: usize },
    Dropout { rate: f64 },
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub layer_type: LayerType,
    pub activation: Option<ActivationType>,
}

impl Layer {
    pub fn new(layer_type: LayerType, activation: Option<ActivationType>) -> Self {
        Self {
            layer_type,
            activation,
        }
    }

    /// Calculate approximate parameter count for this layer
    pub fn parameter_count(&self, input_size: usize) -> usize {
        match &self.layer_type {
            LayerType::Dense { units } => input_size * units + units,
            LayerType::Convolution { filters, kernel_size } => {
                filters * (kernel_size * kernel_size * input_size + 1)
            }
            LayerType::Pooling { .. } => 0,
            LayerType::Dropout { .. } => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkArchitecture {
    pub layers: Vec<Layer>,
    pub input_shape: Vec<usize>,
    pub estimated_performance: Option<f64>,
}

impl NetworkArchitecture {
    pub fn new(input_shape: Vec<usize>) -> Self {
        Self {
            layers: Vec::new(),
            input_shape,
            estimated_performance: None,
        }
    }

    /// Calculate total parameters in the network
    pub fn total_parameters(&self) -> usize {
        let mut current_size = self.input_shape[0];
        let mut total = 0;

        for layer in &self.layers {
            total += layer.parameter_count(current_size);
            match &layer.layer_type {
                LayerType::Dense { units } => current_size = *units,
                LayerType::Convolution { filters, .. } => current_size = *filters,
                LayerType::Pooling { .. } => (), // Size changes but no parameters
                LayerType::Dropout { .. } => (), // No change in size or parameters
            }
        }
        total
    }
}

pub struct ArchitectureSearcher {
    max_layers: usize,
    min_layers: usize,
    max_params: usize,
    memo: HashMap<Vec<Layer>, f64>,
    layer_options: Vec<Layer>,
}

impl ArchitectureSearcher {
    pub fn new(
        max_layers: usize,
        min_layers: usize,
        max_params: usize,
        layer_options: Vec<Layer>,
    ) -> Self {
        Self {
            max_layers,
            min_layers,
            max_params,
            memo: HashMap::new(),
            layer_options,
        }
    }

    /// Recursively search for optimal architecture
    pub fn search(&mut self, input_shape: Vec<usize>) -> Option<NetworkArchitecture> {
        let mut best_architecture = NetworkArchitecture::new(input_shape.clone());
        let mut best_performance = f64::NEG_INFINITY;

        self.search_recursive(
            Vec::new(),
            &input_shape,
            &mut best_architecture,
            &mut best_performance,
        );

        if best_performance > f64::NEG_INFINITY {
            best_architecture.estimated_performance = Some(best_performance);
            Some(best_architecture)
        } else {
            None
        }
    }

    fn search_recursive(
        &mut self,
        current_layers: Vec<Layer>,
        input_shape: &[usize],
        best_architecture: &mut NetworkArchitecture,
        best_performance: &mut f64,
    ) {
        // Base cases
        if current_layers.len() >= self.max_layers {
            return;
        }

        let current_arch = NetworkArchitecture {
            layers: current_layers.clone(),
            input_shape: input_shape.to_vec(),
            estimated_performance: None,
        };

        // Check parameter budget
        if current_arch.total_parameters() > self.max_params {
            return;
        }

        // Evaluate current architecture if it meets minimum layer requirement
        if current_layers.len() >= self.min_layers {
            if let Some(performance) = self.evaluate_architecture(&current_layers) {
                if performance > *best_performance {
                    *best_performance = performance;
                    best_architecture.layers = current_layers.clone();
                }
            }
        }

        // Try adding each possible layer type
        for layer in &self.layer_options {
            let mut new_layers = current_layers.clone();
            new_layers.push(layer.clone());
            
            // Recursive exploration
            self.search_recursive(
                new_layers,
                input_shape,
                best_architecture,
                best_performance,
            );
        }
    }

    /// Evaluate architecture using a heuristic or simulated performance
    fn evaluate_architecture(&mut self, layers: &[Layer]) -> Option<f64> {
        // Check memoization
        if let Some(&score) = self.memo.get(layers) {
            return Some(score);
        }

        // This is where you would typically:
        // 1. Train a small version of the network
        // 2. Use a surrogate model
        // 3. Apply architecture heuristics
        
        // For demonstration, we'll use a simple heuristic:
        // - Favor moderate depth (penalize too few or too many layers)
        // - Prefer alternating conv/pool layers
        // - Reward gradually decreasing dense layer sizes
        
        let mut score = 0.0;
        
        // Depth scoring
        let depth_factor = -(layers.len() as f64 - 5.0).powi(2) / 10.0;
        score += depth_factor;

        // Layer pattern scoring
        for window in layers.windows(2) {
            match (&window[0].layer_type, &window[1].layer_type) {
                (LayerType::Convolution { .. }, LayerType::Pooling { .. }) => score += 1.0,
                (LayerType::Dense { units: u1 }, LayerType::Dense { units: u2 }) => {
                    if u1 > u2 {
                        score += 0.5;
                    }
                }
                _ => {}
            }
        }

        // Normalize score
        let final_score = (score + 10.0) / 20.0;
        self.memo.insert(layers.to_vec(), final_score);
        
        Some(final_score)
    }
}