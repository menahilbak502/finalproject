

// stats_util.rs


/// Helper function to calculate median.
pub fn median(numbers: &mut [f64]) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    }
 }
 
 
 /// Helper function to calculate standard deviation.
 pub fn standard_deviation(numbers: &[f64], mean: f64) -> f64 {
    let variance: f64 = numbers.iter()
        .map(|value| {
            let diff = mean - (*value as f64);
            diff * diff
        })
        .sum::<f64>() / numbers.len() as f64;
    variance.sqrt()
 }
 