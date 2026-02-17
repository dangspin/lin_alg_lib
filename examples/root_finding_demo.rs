use lin_alg_lab::root_finding::{find_root_bisection, find_root_newton, RootError};

fn main() -> Result<(), RootError> {
    // 例子：解 x^3 - x - 2 = 0
    // 这个方程在 x ≈ 1.521... 处有一个实根
    let f = |x: f64| x * x * x - x - 2.0;
    let df = |x: f64| 3.0 * x * x - 1.0;

    let a = 1.0;
    let b = 2.0;
    let x0 = 1.5;

    let tol = 1e-8;
    let max_iter = 50;

    let root_bisect = find_root_bisection(f, a, b, tol, max_iter)?;
    let root_newton = find_root_newton(f, df, x0, tol, max_iter)?;

    let exact = 1.521_379_706_804_567_6_f64; // 你可以当作参考值

    println!("Bisection root ≈ {}, error = {}", root_bisect, (root_bisect - exact).abs());
    println!("Newton    root ≈ {}, error = {}", root_newton, (root_newton - exact).abs());

    // 简单 sanity check：两种方法都应该比较接近 exact
    assert!((root_bisect - exact).abs() < 1e-4);
    assert!((root_newton - exact).abs() < 1e-8);

    // 你也可以自己再试一个没有异号的区间，看能否正确返回 RootError::NoSignChange
    match find_root_bisection(f, 10.0, 11.0, tol, max_iter) {
        Err(RootError::NoSignChange) => {}
        Ok(_) => return Err(RootError::NoSignChange),
        Err(e) => return Err(e),
    }

    println!("root_finding_demo passed");
    Ok(())
}
