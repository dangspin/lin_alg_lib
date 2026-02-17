/// 求根过程中可能发生的错误
#[derive(Debug)]
pub enum RootError {
    /// 在区间 [a, b] 上 f(a) 和 f(b) 没有异号，不能保证有根
    NoSignChange,
    /// 在给定迭代次数内没有收敛
    NoConvergence,
}

/// 使用二分法在 [a, b] 上寻找 f(x) = 0 的根
///
/// 要求：
/// - 初始区间 [a, b] 必须满足 f(a) * f(b) <= 0
/// - 收敛准则可以用区间长度或 |f(m)| < tol
pub fn find_root_bisection<F>(
    f: F,
    mut a: f64,
    mut b: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, RootError>
where
    F: Fn(f64) -> f64,
{
    if f(a) * f(b) > 0.0 {
        return Err(RootError::NoSignChange);
    }

    for _ in 0..max_iter {
        let mid = (a + b) / 2.0;
        let fm = f(mid);

        if fm.abs() < tol {
            return Ok(mid);
        }

        if fm * f(a) > 0.0 {
            a = mid;
        } else {
            b = mid;
        }
    }

    Err(RootError::NoConvergence)
}

/// 使用牛顿法寻找 f(x) = 0 的根
///
/// 参数：
/// - f: 目标函数
/// - df: 导数
/// - x0: 初始迭代点
pub fn find_root_newton<F, DF>(
    f: F,
    df: DF,
    mut x: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, RootError>
where
    F: Fn(f64) -> f64,
    DF: Fn(f64) -> f64,
{
    for _ in 0..max_iter {
        let fx = f(x);
        if fx.abs() < tol {
            return Ok(x);
        }

        let dfx = df(x);
        if dfx.abs() < tol {
            return Err(RootError::NoConvergence);
        }

        x = x - fx / dfx;
    }

    Err(RootError::NoConvergence)
}