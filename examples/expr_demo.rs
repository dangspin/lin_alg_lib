use lin_alg_lab::expr::{EvalError, Expr, eval, to_infix};
use std::collections::HashMap;

fn main() -> Result<(), EvalError> {
    // 假设你定义的 Expr 至少包含：
    // - Expr::Const(f64)
    // - Expr::Var(String)
    // - Expr::Add(Box<Expr>, Box<Expr>)
    // - Expr::Mul(Box<Expr>, Box<Expr>)
    // - Expr::Neg(Box<Expr>)  （这次不一定用到）

    // 构造表达式：(x + 1) * y
    //
    // 注意：这里假设你给 Expr 实现了类似：
    // - Expr::Const(...)
    // - Expr::Var("x".to_string())
    //
    // 你也可以额外实现一些辅助构造函数（不是必须）。
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(1.0)),
        )),
        Box::new(Expr::Var("y".to_string())),
    );

    // 构造变量环境：x = 2, y = 3
    let mut env = HashMap::new();
    env.insert("x".to_string(), 2.0);
    env.insert("y".to_string(), 3.0);

    // 打印中缀表示
    let s = to_infix(&expr);
    println!("expr = {}", s);

    // 求值：在 x=2,y=3 下，(x + 1) * y = (2+1)*3 = 9
    let v = eval(&expr, &env)?;
    println!("value = {}", v);

    assert!((v - 9.0).abs() < 1e-10);

    // 再构造一个包含未知变量的表达式，测试 UnknownVar 分支
    let expr2 = Expr::Add(
        Box::new(Expr::Var("z".to_string())),
        Box::new(Expr::Const(1.0)),
    );

    match eval(&expr2, &env) {
        Ok(v) => {
            println!("expr2 value = {}", v);
            // 这里理论上不应该走到 Ok 分支
        }
        Err(e) => {
            println!("expr2 eval error: {:?}", e);
            // 你可以在这里匹配 EvalError::UnknownVar(name)
        }
    }

    println!("Expr demo done.");
    Ok(())
}
