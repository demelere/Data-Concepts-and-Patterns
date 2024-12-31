use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct TrackedObject {
    pub id: usize,
    pub current_bbox: BoundingBox,
    pub history: VecDeque<BoundingBox>,
    pub velocity: (f32, f32),
    pub confidence: f32,
}

pub struct ObjectTracker {
    pub objects: Vec<TrackedObject>,
    window_size: usize,
    next_id: usize,
    iou_threshold: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn calculate_iou(&self, other: &BoundingBox) -> f32 {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x2 < x1 || y2 < y1 {
            return 0.0;
        }

        let intersection = (x2 - x1) * (y2 - y1);
        let area1 = self.width * self.height;
        let area2 = other.width * other.height;
        let union = area1 + area2 - intersection;

        intersection / union
    }
}

impl TrackedObject {
    pub fn new(id: usize, bbox: BoundingBox, window_size: usize) -> Self {
        let mut history = VecDeque::with_capacity(window_size);
        history.push_back(bbox.clone());
        
        Self {
            id,
            current_bbox: bbox,
            history,
            velocity: (0.0, 0.0),
            confidence: 1.0,
        }
    }

    pub fn update(&mut self, new_bbox: BoundingBox) {
        if let Some(old_bbox) = self.history.front() {
            self.velocity = (
                (new_bbox.x - old_bbox.x) / self.history.len() as f32,
                (new_bbox.y - old_bbox.y) / self.history.len() as f32,
            );
        }

        if self.history.len() >= self.history.capacity() {
            self.history.pop_front();
        }
        self.history.push_back(new_bbox.clone());
        
        self.current_bbox = new_bbox;
        self.confidence = self.calculate_tracking_confidence();
    }

    pub fn predict_next_position(&self) -> BoundingBox {
        BoundingBox {
            x: self.current_bbox.x + self.velocity.0,
            y: self.current_bbox.y + self.velocity.1,
            width: self.current_bbox.width,
            height: self.current_bbox.height,
        }
    }

    fn calculate_tracking_confidence(&self) -> f32 {
        if self.history.len() < 2 {
            return 1.0;
        }

        let mut total_iou = 0.0;
        let mut prev_bbox = &self.history[0];
        
        for bbox in self.history.iter().skip(1) {
            total_iou += prev_bbox.calculate_iou(bbox);
            prev_bbox = bbox;
        }

        total_iou / (self.history.len() - 1) as f32
    }
}

impl ObjectTracker {
    pub fn new(window_size: usize, iou_threshold: f32) -> Self {
        Self {
            objects: Vec::new(),
            window_size,
            next_id: 0,
            iou_threshold,
        }
    }

    pub fn update(&mut self, detections: Vec<BoundingBox>) {
        let mut unmatched_detections = detections.clone();
        let mut matched_indices = Vec::new();

        for object in &mut self.objects {
            let predicted_bbox = object.predict_next_position();
            
            if let Some((best_idx, best_iou)) = unmatched_detections
                .iter()
                .enumerate()
                .map(|(idx, det)| (idx, predicted_bbox.calculate_iou(det)))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            {
                if best_iou > self.iou_threshold {
                    object.update(unmatched_detections[best_idx].clone());
                    matched_indices.push(best_idx);
                }
            }
        }

        for idx in matched_indices.iter().rev() {
            unmatched_detections.swap_remove(*idx);
        }

        for detection in unmatched_detections {
            self.objects.push(TrackedObject::new(
                self.next_id,
                detection,
                self.window_size,
            ));
            self.next_id += 1;
        }

        self.objects.retain(|obj| obj.confidence > 0.3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_iou() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 2.0, 2.0);
        let bbox2 = BoundingBox::new(1.0, 1.0, 2.0, 2.0);
        
        let iou = bbox1.calculate_iou(&bbox2);
        assert!(iou > 0.0 && iou < 1.0);
    }

    #[test]
    fn test_object_tracking() {
        let mut tracker = ObjectTracker::new(5, 0.5);
        
        let detections = vec![
            BoundingBox::new(0.0, 0.0, 1.0, 1.0),
            BoundingBox::new(5.0, 5.0, 1.0, 1.0),
        ];
        
        tracker.update(detections);
        assert_eq!(tracker.objects.len(), 2);
    }
}