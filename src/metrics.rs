use num::traits::{ToPrimitive, Zero};

/// 算術平均を求める
///
/// # Arguments
/// * `data` - データ
///
/// # Examples
///
/// ```
/// use statistics_by_rust::metrics::mean;
/// let data = vec![1, 2, 3, 4, 5];
/// assert_eq!(3.0, mean::<usize>(&data));
/// ```
pub fn mean<T>(data: &[T]) -> f64
where
    T: Copy + std::ops::AddAssign<T> + Zero + ToPrimitive,
{
    let mut total = T::zero();
    for v in data.iter() {
        total += *v;
    }
    total.to_f64().unwrap() / (data.len() as f64)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mean() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(3.0, mean::<usize>(&data));
        let data = vec![4.0, 4.0, 4.0, 4.0, 5.0];
        assert_eq!(4.20, mean::<f64>(&data));
    }
}
