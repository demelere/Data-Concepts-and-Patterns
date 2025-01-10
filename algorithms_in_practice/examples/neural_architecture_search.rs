use algorithms_in_practice::algorithms::recursion::{
    ActivationType, ArchitectureSearcher, Layer, LayerType, NetworkArchitecture,
};

fn main() {
    // Define input shape (e.g., for MNIST: 28x28x1)
    let input_shape = vec![784]; // Flattened input

    // Define possible layer configurations
    let layer_options = vec![
        // Dense layers with different sizes
        Layer::new(LayerType::Dense { units: 512 }, Some(ActivationType::ReLU)),
        Layer::new(LayerType::Dense { units: 256 }, Some(ActivationType::ReLU)),
        Layer::new(LayerType::Dense { units: 128 }, Some(ActivationType::ReLU)),
        
        // Convolutional layers
        Layer::new(
            LayerType::Convolution {
                filters: 32,
                kernel_size: 3,
            },
            Some(ActivationType::ReLU),
        ),
        Layer::new(
            LayerType::Convolution {
                filters: 64,
                kernel_size: 3,
            },
            Some(ActivationType::ReLU),
        ),
        
        // Pooling layers
        Layer::new(LayerType::Pooling { pool_size: 2 }, None),
        
        // Dropout layers
        Layer::new(LayerType::Dropout { rate: 0.3 }, None),
    ];

    // Create architecture searcher
    let mut searcher = ArchitectureSearcher::new(
        max_layers: 8,    // Maximum depth
        min_layers: 3,    // Minimum depth
        max_params: 1_000_000, // Parameter budget
        layer_options,
    );

    println!("Starting Neural Architecture Search...");
    println!("Input shape: {:?}", input_shape);
    println!("Constraints:");
    println!("  - Max layers: {}", searcher.max_layers);
    println!("  - Min layers: {}", searcher.min_layers);
    println!("  - Max parameters: {}", searcher.max_params);
    println!("\nSearching...\n");

    // Perform architecture search
    match searcher.search(input_shape) {
        Some(architecture) => {
            println!("Found optimal architecture!");
            println!("Estimated performance score: {:.4}", 
                    architecture.estimated_performance.unwrap());
            println!("\nArchitecture details:");
            
            print_architecture(&architecture);
        }
        None => println!("No valid architecture found within constraints!"),
    }
}

fn print_architecture(architecture: &NetworkArchitecture) {
    println!("Total parameters: {}", architecture.total_parameters());
    println!("\nLayer structure:");
    
    for (i, layer) in architecture.layers.iter().enumerate() {
        let layer_str = match &layer.layer_type {
            LayerType::Dense { units } => 
                format!("Dense (units={})", units),
            LayerType::Convolution { filters, kernel_size } => 
                format!("Conv2D (filters={}, kernel={}x{})", 
                       filters, kernel_size, kernel_size),
            LayerType::Pooling { pool_size } => 
                format!("MaxPooling (size={}x{})", pool_size, pool_size),
            LayerType::Dropout { rate } => 
                format!("Dropout (rate={:.1})", rate),
        };

        let activation_str = match &layer.activation {
            Some(act) => match act {
                ActivationType::ReLU => "ReLU",
                ActivationType::Sigmoid => "Sigmoid",
                ActivationType::Tanh => "Tanh",
            },
            None => "None",
        };

        println!("{}. {} (Activation: {})", i + 1, layer_str, activation_str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_search() {
        let layer_options = vec![
            Layer::new(LayerType::Dense { units: 64 }, Some(ActivationType::ReLU)),
            Layer::new(LayerType::Dense { units: 32 }, Some(ActivationType::ReLU)),
        ];

        let mut searcher = ArchitectureSearcher::new(4, 2, 100_000, layer_options);
        let result = searcher.search(vec![784]);
        
        assert!(result.is_some());
        let architecture = result.unwrap();
        assert!(architecture.layers.len() >= 2);
    }

    #[test]
    fn test_parameter_constraint() {
        let layer_options = vec![
            Layer::new(LayerType::Dense { units: 1000 }, Some(ActivationType::ReLU)),
        ];

        let mut searcher = ArchitectureSearcher::new(4, 2, 1000, layer_options);
        let result = searcher.search(vec![784]);
        
        // Should fail due to parameter constraint
        assert!(result.is_none());
    }
}