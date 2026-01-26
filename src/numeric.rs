/// 使用梯形法则（trapezoidal rule）近似计算定积分：

/// 要求：
/// - 如果 xs.len() < 2 或 xs.len() != ys.len()，返回 0.0（或者你可以选择 panic，现在为了简化先返回 0）
///
/// 建议实现：
/// - 用 xs.windows(2) 和 ys.windows(2) 组合迭代每一小段
pub fn trapezoidal_integrate(xs: &[f64], ys: &[f64]) -> f64 {
    if xs.len() < 2 || xs.len() != ys.len() {
        return 0.0;
    }

    let mut integral = 0.0;
    
    for (x, y) in xs.windows(2).zip(ys.windows(2)) {
        let x0 = x[0];
        let x1 = x[1];
        let y0 = y[0];
        let y1 = y[1];
        integral += (x1 - x0) * (y0 + y1) / 2.0;
    }

    integral
}

/// 使用一阶差分近似导数：f'(x[i]) ≈ (y[i+1] - y[i]) / (x[i+1] - x[i])
///
/// 输入：
/// - xs: 自变量样本点
/// - ys: 函数值样本点
///
/// 输出：
/// - Vec<f64>，长度为 n - 1（如果 n = xs.len()）
///   - 第 i 个元素是对区间 [x[i], x[i+1]] 上导数的近似
///
/// 要求：
/// - xs.len() 和 ys.len() 相等，否则可以返回空 Vec 或者直接 panic（你自己决定一个策略，并在注释里写清楚）
/// - 如果 len < 2，返回空 Vec
///
/// 建议实现：
/// - 使用 xs.windows(2).zip(ys.windows(2))
///   - 对于每个 ((x[i], x[i+1]), (y[i], y[i+1])) 计算差分
pub fn finite_differences(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    if xs.len() < 2 || xs.len() != ys.len() {
        return Vec::new();
    }

    let mut diffs = Vec::new();

    for (x, y) in xs.windows(2).zip(ys.windows(2)) {
        let x0 = x[0];
        let x1 = x[1];
        let y0 = y[0];
        let y1 = y[1];
        diffs.push((y1 - y0) / (x1 - x0));
    }

    diffs
}