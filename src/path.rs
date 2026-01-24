use crate::vector::{Vector2, distance};

/// 计算一条折线的总长度。
///
/// points: 折线上的一系列点，按顺序给出。
///
/// - 如果点数 < 2，则长度为 0。
/// - 否则，总长度 = sum over i: distance(points[i], points[i+1])
///
/// 建议实现方式：
/// - 使用切片自带的 `points.windows(2)`
///   - 每个窗口是 `&[Vector2]`，长度固定为 2
///   - window[0] 和 window[1] 就是一段线段的两个端点
pub fn polyline_length(points: &[Vector2]) -> f64 {
    if points.len() < 2 {
        return 0.0;
    }
    points.windows(2)
        .map(|p| {distance(&p[0], &p[1])})
        .sum()
}

/// 计算从起点到每个点的“累计长度”。
///
/// 返回一个 Vec<f64>，长度和 points 一样：
/// - 如果 points 为空，返回空 Vec；
/// - 假设 points = [p0, p1, p2, p3]
///   - 输出 = [0.0, |p0-p1|, |p0-p1|+|p1-p2|, |p0-p1|+|p1-p2|+|p2-p3|]
///
/// 提示：
/// - 可以先用 `points.windows(2)` 得到每一段的长度，形成一个迭代器：segments_lengths
/// - 然后用迭代器的 `scan` 或者手写循环，累计这些长度。
pub fn cumulative_lengths(points: &[Vector2]) -> Vec<f64> {
    let mut cumul_len = Vec::new();
    let mut len = 0.0;
    cumul_len.push(len);

    for p in points.windows(2).map(|p| distance(&p[0], &p[1])) {
        len += p;
        cumul_len.push(len);
    }

    cumul_len
}