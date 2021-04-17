//! 統計処理をRustで実装します。

pub mod metrics;

/// 棒グラフを出力する。
///
/// # Arguments
/// * `values` - 座標の列
/// * `labels` - ラベルの列
///
/// # Examples
///
/// ```
/// use statistics_by_rust::bar_chart;
/// let values = [1, 2, 3];
/// let labels = ["January", "February", "March"];
/// assert_eq!(
///     vec![
///         "January  |  1 | #",
///         "February |  2 | ##",
///         "March    |  3 | ###"
///     ].join("\n"),
///     bar_chart(&values, &labels)
/// );
/// ```
pub fn bar_chart(values: &[usize], labels: &[&str]) -> String {
    let n = values.len();
    assert_eq!(n, labels.len());
    let max_label_len = labels.iter().map(|x| x.len()).max().unwrap();
    let max_value = *values.iter().max().unwrap() as f64;
    let max_value_len = max_value.log(10.0).ceil() as usize + 1;

    labels
        .iter()
        .zip(values.iter())
        .map(|(k, v)| {
            format!(
                "{0:<label_len$} | {1:>value_len$} | {2}",
                k,
                v,
                "#".repeat(*v),
                label_len = max_label_len,
                value_len = max_value_len
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// [min, max)の範囲内でbin_sizeごとに階級を切って度数分布を集計する。
///
/// # Arguments
/// * `data` - データ
/// * `bin_size` - 階級幅
/// * `min` - 下限が最小の階級の下限
/// * `max` - 上限が最大の階級の上限
///
/// # Examples
///
/// ```
/// use statistics_by_rust::frequency_distribution;
/// let data = [1, 2, 3, 4, 5, 6, 7];
/// let bin_size = 4;
/// let min = 1;
/// let max = 10;
/// let (thresholds, frequencies) = frequency_distribution::<usize>(&data, bin_size, min, max);
/// assert_eq!(vec![(1, 5), (5, 9), (9, 13)], thresholds);
/// assert_eq!(vec![4, 3, 0], frequencies);
/// ```
pub fn frequency_distribution<T>(
    data: &[T],
    bin_size: T,
    min: T,
    max: T,
) -> (Vec<(T, T)>, Vec<usize>)
where
    T: Copy + std::cmp::PartialOrd + std::ops::Add<Output = T> + std::ops::AddAssign,
{
    let mut bins = vec![];
    let mut labels = vec![];
    let mut lower_threshold = min;
    let mut upper_threshold = min + bin_size;
    while lower_threshold < max {
        labels.push((lower_threshold, upper_threshold));
        bins.push(0);
        lower_threshold = upper_threshold;
        upper_threshold += bin_size;
    }
    let n_bin = bins.len();
    for &v in data.iter() {
        if v < min || max < v {
            continue;
        }
        let mut threshold = min + bin_size;
        for bin in bins.iter_mut().take(n_bin) {
            if v < threshold {
                *bin += 1;
                break;
            }
            threshold += bin_size;
        }
    }
    (labels, bins)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bar_chart() {
        assert_eq!(
            "a   |   1 | #\nbb  |   3 | ###\ncde |  12 | ############".to_string(),
            bar_chart(&[1, 3, 12], &["a", "bb", "cde"])
        );
        assert_eq!(
            "a     |   1 | #\nbbbbb |   3 | ###\ncde   | 100 | ####################################################################################################".to_string(),
            bar_chart(&[1,3,100], &["a", "bbbbb", "cde"])
        );
    }
    #[test]
    fn test_frequency_distribution() {
        assert_eq!(
            (vec![(0, 3), (3, 6), (6, 9)], vec![2, 3, 2]),
            frequency_distribution::<usize>(&[1, 2, 3, 4, 5, 6, 7], 3, 0, 9)
        );
        assert_eq!(
            (vec![(1, 5), (5, 9), (9, 13)], vec![4, 3, 0]),
            frequency_distribution::<usize>(&[1, 2, 3, 4, 5, 6, 7], 4, 1, 10)
        );
        assert_eq!(
            (vec![(1.0, 5.0), (5.0, 9.0), (9.0, 13.0)], vec![4, 3, 0]),
            frequency_distribution::<f64>(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], 4.0, 1.0, 10.0)
        );
    }
}
