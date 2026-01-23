use std::ops::{Add, Sub, Neg, Mul};
// src/vector.rs
// 你需要自己在这里定义 struct Vector2：
// - 例如：表示一个二维向量 (x, y)
// - 字段类型建议用 f64
// - 你可以自己决定是否实现 Debug / Clone / Copy 等 trait
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x:f64, y:f64) -> Self {
        Self {x, y}
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// 返回归一化后的向量（单位向量）。
    ///
    /// 约定：如果这个向量太接近零向量，则返回 None。
    /// （因为“零向量的方向”在数学上是未定义的）
    pub fn normalize(&self) -> Option<Vector2> {
        let len_sq = self.length_sq();
        if len_sq < 1e-10 {
            return None;
        }
        let len = f64::sqrt(len_sq);
        Some(Vector2::new(self.x / len, self.y / len))
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs * self
    }
}

/// 计算两个二维向量的点积：a · b = a.x * b.x + a.y * b.y
pub fn dot(a: &Vector2, b: &Vector2) -> f64 {
    a.x * b.x + a.y * b.y
}

/// 计算二维向量的欧几里得范数（长度）：‖v‖ = sqrt(v.x^2 + v.y^2)
///
/// 提示：可用 `f64::sqrt`。
pub fn norm(v: &Vector2) -> f64 {
    f64::sqrt(dot(v, v))
}

/// 计算两点（向量）之间的欧几里得距离：‖a - b‖
pub fn distance(a: &Vector2, b: &Vector2) -> f64 {
    let diff = *a - *b;
    norm(&diff)
}

/// 计算向量 `v` 在 `basis` 上的投影：
///
///   proj_basis(v) = ( (v · basis) / (basis · basis) ) * basis
///
/// 要点：
/// - 参数使用 &Vector2，返回一个新的 Vector2（有自己所有权）
/// - 注意处理 `basis` 是零向量的情况：
///   - 本练习先简单处理：如果 basis 是零向量，可以直接返回一个全 0 的 Vector2
///   - 或者你也可以选择返回 `v` 自己，完全由你决定，只要和 main 里的预期一致即可
pub fn project_onto(v: &Vector2, basis: &Vector2) -> Vector2 {
    let basis_norm = norm(basis);
    if basis_norm.abs() < 1e-10 {
        return Vector2::new(0.0, 0.0);
    }
    let proj = dot(v, basis) / dot(basis, basis);
    Vector2 {
        x: proj * basis.x,
        y: proj * basis.y,
    }
}