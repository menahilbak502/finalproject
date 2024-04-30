//stat_util.rs

/// Helper function to calculate the median of a list of numbers.
pub fn median(numbers: &mut [f64]) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());  // Sort the numbers in ascending order.
    let mid = numbers.len() / 2;  // Compute the middle index of the sorted list.
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0  // If even, return the average of the two middle numbers.
    } else {
        numbers[mid]  // If odd, return the middle number.
    }
}
 
/// Helper function to calculate the standard deviation of a set of numbers.
pub fn standard_deviation(numbers: &[f64], mean: f64) -> f64 {
    let variance: f64 = numbers.iter()
        .map(|value| {
            let diff = mean - (*value as f64);  // Calculate the difference from the mean for each number.
            diff * diff  // Square the difference.
        })
        .sum::<f64>() / numbers.len() as f64;  // Calculate the average of the squared differences (variance).
    variance.sqrt()  // Return the square root of the variance (standard deviation).
}
