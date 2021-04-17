//! 統計処理をRustで実装します。

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
/// println!("{}", bar_chart(&values, &labels));
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
}
