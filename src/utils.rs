use std::ops::Range;

pub fn map_range(value: f32, from: Range<f32>, to: Range<f32>) -> f32 {
    let from_span = from.end - from.start;
    let to_span = to.end - to.start;
    if from_span == 0.0 {
        return to.start; // Avoid division by zero
    }
    let normalized_value = (value - from.start) / from_span;
    to.start + normalized_value * to_span
}
