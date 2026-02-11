use lin_alg_lab::iter_utils::detect_1d_outliers_zscore;

fn main() {
    // 构造一段简单数据：大部分是 0 附近，夹杂几个尖峰
    let data = vec![
        0.1, 0.0, -0.2, 0.05, 0.1, 5.0, // 明显的正向异常
        0.2, 0.0, 0.1, -4.5, // 明显的负向异常
        0.0, 0.1,
    ];

    let window_size = 5;
    let threshold = 3.0;

    println!("data = {:?}", data);

    let flags = detect_1d_outliers_zscore(&data, window_size, threshold);
    println!("outlier flags = {:?}", flags);

    // 简单打印出哪些位置被标记为异常
    for (i, (&x, &flag)) in data.iter().zip(flags.iter()).enumerate() {
        if flag {
            println!("outlier at index {}: value = {}", i, x);
        }
    }

    // 简单检查：我们至少期望 index 5 被标记为异常（对应 5.0）
    assert_eq!(flags.len(), data.len());
    assert!(flags[5], "index 5 (value 5.0) should be outlier");

    println!("Outlier detection demo passed.");
}
