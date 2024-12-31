use algorithms_in_practice::algorithms::sliding_window::{BoundingBox, ObjectTracker};

fn main() {
    let mut tracker = ObjectTracker::new(5, 0.5);
    
    // Simulate some detections across frames
    let detections_frame1 = vec![
        BoundingBox::new(100.0, 100.0, 50.0, 50.0),
        BoundingBox::new(200.0, 150.0, 40.0, 60.0),
    ];
    
    let detections_frame2 = vec![
        BoundingBox::new(105.0, 102.0, 50.0, 50.0),  // Slightly moved
        BoundingBox::new(210.0, 155.0, 40.0, 60.0),  // Slightly moved
    ];
    
    // Process frames
    tracker.update(detections_frame1);
    println!("Frame 1 - Number of tracked objects: {}", tracker.objects.len());
    
    tracker.update(detections_frame2);
    println!("Frame 2 - Number of tracked objects: {}", tracker.objects.len());
    
    // Print tracking results
    for object in &tracker.objects {
        println!(
            "Object {}: Position: ({}, {}), Velocity: ({:.2}, {:.2}), Confidence: {:.2}",
            object.id,
            object.current_bbox.x,
            object.current_bbox.y,
            object.velocity.0,
            object.velocity.1,
            object.confidence
        );
    }
}