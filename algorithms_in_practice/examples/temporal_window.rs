use algorithms_in_practice::algorithms::sliding_window::{WindowConfig, TemporalWindowProcessor};

#[derive(Debug, Clone)]
struct Frame {
    timestamp: u64,
    data: Vec<f32>,
}

impl Frame {
    fn new(timestamp: u64, data: Vec<f32>) -> Self {
        Self { timestamp, data }
    }

    fn process_window(frames: &[Frame]) -> Frame {
        // For this example, we'll average the data across frames in the window
        let avg_data: Vec<f32> = if frames.is_empty() {
            Vec::new()
        } else {
            let data_len = frames[0].data.len();
            let mut sums = vec![0.0; data_len];
            
            // Sum all values
            for frame in frames {
                for (sum, &value) in sums.iter_mut().zip(frame.data.iter()) {
                    *sum += value;
                }
            }
            
            // Calculate averages
            sums.iter().map(|&sum| sum / frames.len() as f32).collect()
        };

        // Use the middle frame's timestamp if possible
        let mid_idx = frames.len() / 2;
        let timestamp = frames.get(mid_idx)
            .map(|f| f.timestamp)
            .unwrap_or(0);

        Frame::new(timestamp, avg_data)
    }
}

fn main() {
    // Create some sample frame data
    let sequence_length = 20;
    let frames: Vec<Frame> = (0..sequence_length)
        .map(|i| {
            let data = vec![i as f32, (i * 2) as f32, (i * 3) as f32];
            Frame::new(i as u64, data)
        })
        .collect();

    // Configure temporal window processing
    let config = WindowConfig {
        dilations: vec![25, 10, 1],           // Start with large dilation, end with fine detail
        snippet_lengths: vec![3, 3, 3],       // Use 3-frame windows throughout
        strides: vec![1, 1, 1],              // Process every frame
        cap_dilation: true,                   // Automatically adjust dilations for sequence length
    };

    let processor = TemporalWindowProcessor::new(config);

    println!("Processing sequence of {} frames...\n", sequence_length);

    // Process the sequence
    let result = processor.process_sequence(&frames, Frame::process_window);

    // Print results for each scale
    println!("Processing Results:");
    println!("Effective dilations: {:?}\n", result.effective_dilations);

    for (scale_idx, dilation) in result.effective_dilations.iter().enumerate() {
        println!("Scale {} (dilation = {}):", scale_idx + 1, dilation);
        println!("Window indices:");
        for window in &result.window_indices[scale_idx] {
            println!("  {:?}", window);
        }
        println!();
    }

    // Verify frame coverage
    let coverage_ok = processor.verify_coverage(
        sequence_length,
        &result.window_indices.concat()
    );
    println!("All frames covered: {}", coverage_ok);

    // Print some processed data
    println!("\nSample processed results:");
    for (i, frame) in result.processed_data.iter().take(5).enumerate() {
        println!("Result {}: t={}, data={:?}", i + 1, frame.timestamp, frame.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_processing() {
        let frames: Vec<Frame> = (0..10)
            .map(|i| Frame::new(i, vec![i as f32]))
            .collect();

        let config = WindowConfig {
            dilations: vec![1],
            snippet_lengths: vec![3],
            strides: vec![1],
            cap_dilation: true,
        };

        let processor = TemporalWindowProcessor::new(config);
        let result = processor.process_sequence(&frames, Frame::process_window);

        assert!(!result.processed_data.is_empty());
        assert_eq!(result.effective_dilations, vec![1]);
    }

    #[test]
    fn test_dilation_capping() {
        let frames: Vec<Frame> = (0..5)
            .map(|i| Frame::new(i, vec![i as f32]))
            .collect();

        let config = WindowConfig {
            dilations: vec![10],  // Too large for sequence
            snippet_lengths: vec![3],
            strides: vec![1],
            cap_dilation: true,
        };

        let processor = TemporalWindowProcessor::new(config);
        let result = processor.process_sequence(&frames, Frame::process_window);

        assert!(result.effective_dilations[0] < 10);
    }

    #[test]
    fn test_coverage() {
        let frames: Vec<Frame> = (0..10)
            .map(|i| Frame::new(i, vec![i as f32]))
            .collect();

        let config = WindowConfig {
            dilations: vec![1],
            snippet_lengths: vec![3],
            strides: vec![1],
            cap_dilation: true,
        };

        let processor = TemporalWindowProcessor::new(config);
        let result = processor.process_sequence(&frames, Frame::process_window);

        assert!(processor.verify_coverage(
            frames.len(),
            &result.window_indices.concat()
        ));
    }
}