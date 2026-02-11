use std::collections::HashMap;

/// 练习目标：
/// - 定义一个表达式 AST：Expr
/// - 定义一个求值错误类型：EvalError
/// - 实现：
///   - eval(expr, env) -> Result<f64, EvalError>
///   - to_infix(expr) -> String
#[derive(Debug, Clone)]
pub enum Expr {
    Const(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Debug)]
pub enum EvalError {
    UnknownVar(String),
    DivByZero,
}

/// 在给定变量环境 env 下，对表达式 expr 求值。
///
/// env: HashMap<String, f64>，把变量名映射到数值。
///
/// 要求：
/// - 对于 Const(v)：返回 Ok(v)
/// - 对于 Var(name)：
///   - 如果 env 里有这个 key，返回对应的值；
///   - 否则返回 Err(EvalError::UnknownVar(name.clone()))
/// - 对于 Add(a, b)：递归调用 eval(a, env) 和 eval(b, env)，相加；
/// - 对于 Mul(a, b)：递归调用 eval(a, env) 和 eval(b, env)，相乘；
/// - 对于 Neg(e)：返回 -eval(e, env)。
///
/// 提示：
/// - 非常适合用 match + 递归；
/// - 建议使用 ? 运算符来传递 Err。
pub fn eval(expr: &Expr, env: &HashMap<String, f64>) -> Result<f64, EvalError> {
    match expr {
        Expr::Const(v) => Ok(*v),
        Expr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| EvalError::UnknownVar(name.clone())),
        Expr::Add(a, b) => {
            let a = eval(a, env)?;
            let b = eval(b, env)?;
            Ok(a + b)
        }
        Expr::Mul(a, b) => {
            let a = eval(a, env)?;
            let b = eval(b, env)?;
            Ok(a * b)
        }
        Expr::Neg(e) => {
            let e = eval(e, env)?;
            Ok(-e)
        }
    }
}

/// 把表达式 AST 转成中缀字符串形式。
///
/// 例如（具体格式你可以自己决定，只要 example 里能对得上）：
/// - Const(1.23)           -> "1.23"
/// - Var("x")              -> "x"
/// - Add(Const(1), Var("x"))
///   - 可能打印成 "(1 + x)" 或 "1 + x" 都可以；
/// - Mul(Add(Const(1), Var("x")), Const(2))
///   - 建议加括号避免歧义，比如 "(1 + x) * 2"
///
/// 提示：
/// - 同样用 match + 递归；
/// - 用 String::from / format! 来拼接字符串。
pub fn to_infix(expr: &Expr) -> String {
    match expr {
        Expr::Const(v) => format!("{}", v),
        Expr::Var(name) => name.clone(),
        Expr::Add(a, b) => format!("({} + {})", to_infix(a), to_infix(b)),
        Expr::Mul(a, b) => format!("({} * {})", to_infix(a), to_infix(b)),
        Expr::Neg(e) => format!("(-{})", to_infix(e)),
    }
}
