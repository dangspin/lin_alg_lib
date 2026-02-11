use lin_alg_lab::numeric::{finite_differences, trapezoidal_integrate};

fn main() {
    // 用均匀网格在 [0, 1] 上采样 f(x) = x^2
    let n = 11;
    let xs: Vec<f64> = (0..n)
        .map(|i| i as f64 / (n as f64 - 1.0)) // 0, 0.1, 0.2, ..., 1.0
        .collect();
    let ys: Vec<f64> = xs.iter().map(|&x| x * x).collect();

    println!("xs = {:?}", xs);
    println!("ys = {:?}", ys);

    // 1. 用梯形法则近似积分 ∫_0^1 x^2 dx = 1/3
    let approx_int = trapezoidal_integrate(&xs, &ys);
    println!("approx integral = {}", approx_int);
    println!("true integral   = {}", 1.0 / 3.0);

    assert!((approx_int - 1.0 / 3.0).abs() < 1e-2); // 精度要求不需要太苛刻

    // 2. 差分近似导数，f'(x) = 2x
    let derivs = finite_differences(&xs, &ys);
    println!("finite differences (approx derivatives) = {:?}", derivs);

    // 简单检查：在中间点附近，导数应该接近 2x
    // 比如在 x ≈ 0.5 的地方，导数应该接近 1.0
    let mid = n / 2 - 1; // 对应大概在区间 [0.4, 0.5] 或 [0.5, 0.6]
    let x_mid = 0.5;
    let d_mid = derivs[mid];
    println!("approx derivative near x=0.5: {}", d_mid);
    assert!((d_mid - 2.0 * x_mid).abs() < 0.5); // 粗略检查

    println!("All numeric checks passed.");
}
