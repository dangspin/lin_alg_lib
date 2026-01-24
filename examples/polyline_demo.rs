use lin_alg_lab::vector::Vector2;
use lin_alg_lab::path::{polyline_length, cumulative_lengths};

fn main() {
    // 构造一条简单的折线：
    // (0,0) -> (3,0) -> (3,4)
    //
    // 第一段长度 3
    // 第二段长度 4
    // 总长度 7
    let points: Vec<Vector2> = vec![
        (0.0, 0.0).into(),
        (3.0, 0.0).into(),
        (3.0, 4.0).into(),
    ];

    println!("points = {:?}", points);

    let total_len = polyline_length(&points);
    println!("polyline length = {}", total_len);

    let cum = cumulative_lengths(&points);
    println!("cumulative lengths = {:?}", cum);

    // 检查：
    // 总长度应该是 7
    assert!((total_len - 7.0).abs() < 1e-10);

    // 累计长度：
    // p0: 0
    // p1: 3
    // p2: 3 + 4 = 7
    assert_eq!(cum.len(), 3);
    assert!(cum[0].abs() < 1e-10);
    assert!((cum[1] - 3.0).abs() < 1e-10);
    assert!((cum[2] - 7.0).abs() < 1e-10);

    println!("All polyline checks passed.");
}