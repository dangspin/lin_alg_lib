use lin_alg_lab::ode::{solve_ode_euler, solve_ode_rk4};

fn main() {
    // 方程：y' = -y，解析解：y(t) = exp(-t)
    let f = |t: f64, y: f64| {
        let _ = t; // f 在这个例子里不依赖 t
        -y
    };

    let t0 = 0.0;
    let y0 = 1.0;
    let dt = 0.1;
    let n_steps = 10; // 终点 t = 1.0

    let sol_euler = solve_ode_euler(f, t0, y0, dt, n_steps);
    let sol_rk4 = solve_ode_rk4(f, t0, y0, dt, n_steps);

    // 由于 solve_ode 返回的是从 t0 到 t0 + n_steps*dt 共 n_steps+1 个点，
    // last() 就是 t = 1.0 的数值解
    let (_, y_euler) = sol_euler.last().expect("euler solution empty");
    let (_, y_rk4) = sol_rk4.last().expect("rk4 solution empty");

    let exact = (-1.0f64).exp();

    println!("Euler  at t=1: {}, error = {}", y_euler, (y_euler - exact).abs());
    println!("RK4    at t=1: {}, error = {}", y_rk4, (y_rk4 - exact).abs());
    println!("Exact  at t=1: {}", exact);

    // 你可以根据感觉加个断言，比如：
    // assert!((y_rk4 - exact).abs() < (y_euler - exact).abs());

    println!("ode_demo passed");
}