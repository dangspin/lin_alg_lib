/// 一维常微分方程：y' = f(t, y)
/// 这个 trait 表示“单步”步进器：给定 (t, y) 和步长 dt，算出下一个 y。
pub trait OdeStepper {
    fn step<F>(&self, f: F, t: f64, y: f64, dt: f64) -> f64
    where
        F: Fn(f64, f64) -> f64;
}

struct Euler;

impl OdeStepper for Euler {
    fn step<F>(&self, f: F, t: f64, y: f64, dt: f64) -> f64 
    where
        F: Fn(f64, f64) -> f64
    {
        let y1 = y + dt * f(t, y);
        y1
    }
}

struct Rk4;

impl OdeStepper for Rk4 {
    fn step<F>(&self, f: F, t: f64, y: f64, dt: f64) -> f64
    where
        F: Fn(f64, f64) -> f64
    {
        let k1 = dt * f(t, y);
        let k2 = dt * f(t + dt / 2.0, y + k1 / 2.0);
        let k3 = dt * f(t + dt / 2.0, y + k2 / 2.0);
        let k4 = dt * f(t + dt, y + k3);
        let y1 = y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        y1
    }
}



/// 提示：
/// - 你可以定义一个 `struct Euler;`，为它实现 `OdeStepper`，里面写 Euler 显式步进
/// - 也可以定义一个 `struct Rk4;`，实现经典 RK4
pub fn solve_ode<S, F>(
    stepper: &S,
    f: F,
    t0: f64,
    y0: f64,
    dt: f64,
    n_steps: usize,
) -> Vec<(f64, f64)>
where
    S: OdeStepper,
    F: Fn(f64, f64) -> f64,
{
    let mut result = Vec::with_capacity(n_steps);
    let mut t = t0;
    let mut y = y0;

    result.push((t, y));
    for step in 0..n_steps {
        y = stepper.step(&f, t, y, dt);
        t += dt;
        result.push((t, y));
    }

    result
}


/// 提示：
/// - 这个函数可以内部构造一个 Euler 步进器（比如 `Euler`），然后调用 `solve_ode`
/// - 方便用户直接用 Euler，不用自己 new struct
pub fn solve_ode_euler<F>(
    f: F,
    t0: f64,
    y0: f64,
    dt: f64,
    n_steps: usize,
) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    solve_ode(&Euler, f, t0, y0, dt, n_steps)
}

/// 提示：
/// - 同理，这里用 RK4 步进器
pub fn solve_ode_rk4<F>(
    f: F,
    t0: f64,
    y0: f64,
    dt: f64,
    n_steps: usize,
) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    solve_ode(&Rk4, f, t0, y0, dt, n_steps)
}