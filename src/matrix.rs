use crate::vector::Vector2;
use std::ops::Mul;

// 练习目标：
// - 定义一个 2x2 矩阵类型 Matrix2x2
// - 实现：
//   - Matrix2x2::new(...) 构造函数
//   - Matrix2x2::identity() 单位矩阵
//   - Matrix2x2::rotation(theta) 旋转矩阵
//   - Matrix2x2 * Vector2 的乘法（实现 Mul<Vector2>）
#[derive(Debug, Clone, Copy)]
pub struct Matrix2x2 {
    m11: f64, m12: f64,
    m21: f64, m22: f64,
}

impl Matrix2x2 {
    /// 使用四个元素构造一个 2x2 矩阵：
    ///
    /// [ m11  m12 ]
    /// [ m21  m22 ]
    pub fn new(
        m11: f64, m12: f64,
        m21: f64, m22: f64,
    ) -> Self {
        Self { m11, m12, m21, m22 }
    }

    /// 返回 2x2 单位矩阵：
    ///
    /// [ 1  0 ]
    /// [ 0  1 ]
    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// 返回 2D 旋转矩阵：
    ///
    /// [  cosθ  -sinθ ]
    /// [  sinθ   cosθ ]
    pub fn rotation(theta: f64) -> Self {
        Self::new(
            f64::cos(theta), -f64::sin(theta),
            f64::sin(theta), f64::cos(theta),
        )
    }
}

/// 实现矩阵乘以向量：Matrix2x2 * Vector2
///
/// [ a  b ]   [ x ]   [ a*x + b*y ]
/// [ c  d ] * [ y ] = [ c*x + d*y ]
impl Mul<Vector2> for Matrix2x2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Self::Output::new(
            self.m11 * rhs.x() + self.m12 * rhs.y(),
            self.m21 * rhs.x() + self.m22 * rhs.y(),
        )
    }
}

impl<'a> Mul<&'a Vector2> for Matrix2x2 {
    type Output = Vector2;

    fn mul(self, rhs: &'a Vector2) -> Self::Output {
        // 用 rhs.x() / rhs.y()，类型是 &Vector2
        Self::Output::new(
            self.m11 * rhs.x() + self.m12 * rhs.y(),
            self.m21 * rhs.x() + self.m22 * rhs.y(),
        )
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Self) -> Self::Output {
        // 标准 2x2 矩阵乘法
        Self::Output::new(
            self.m11 * rhs.m11 + self.m12 * rhs.m21,
            self.m11 * rhs.m12 + self.m12 * rhs.m22,
            self.m21 * rhs.m11 + self.m22 * rhs.m21,
            self.m21 * rhs.m12 + self.m22 * rhs.m22,
        )
    }
}

impl From<[[f64; 2]; 2]> for Matrix2x2 {
    fn from(arr: [[f64; 2]; 2]) -> Self {
        Self::new(
            arr[0][0], arr[0][1],
            arr[1][0], arr[1][1],
        )
    }
}