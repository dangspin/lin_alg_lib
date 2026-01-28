/// 通用 2D 矩阵类型 Matrix：
/// - 用于表示任意大小的 m×n 实数矩阵
/// - 内部建议使用一维 Vec<f64> 按行优先 (row-major) 存储：
///   data[row * cols + col]

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

#[derive(Debug)]
pub enum MatrixError {
    DimensionMismatch,
    NotSquare,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        if data.len() != rows * cols {
            return Err(MatrixError::DimensionMismatch);
        }
        Ok(Matrix{rows, cols, data})
    }

    /// 创建一个全 0 的矩阵。
    ///
    /// 大小为 rows × cols，所有元素为 0.0。
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix{rows, cols, data: vec![0.0; rows * cols]}
    }

    /// 返回矩阵的行数。
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// 返回矩阵的列数。
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// 内部索引辅助函数：
    /// - 把 (row, col) 映射成 data 里的下标 index
    ///
    /// 行优先布局：
    /// index = row * self.cols + col
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    /// 读取 (row, col) 位置的元素。
    ///
    /// 你可以选择：
    /// - 直接 panic 越界（简单版），或者
    /// - 返回 Result<f64, MatrixError>（更健壮）
    ///
    /// 这里我们先用简单版，假设调用方不越界。
    pub fn get(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        self.data.get(self.index(row, col)).cloned().ok_or(MatrixError::DimensionMismatch)
    }

    /// 设置 (row, col) 位置的元素为 value。
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        let mut index = self.index(row, col);
        self.data[index] = value;
    }

    pub fn transpose(&self) -> Result<Matrix, MatrixError> {
        let mut transposed = Matrix::zeros(self.cols(), self.rows());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                transposed.set(j, i, self.get(i, j)?);
            }
        }
        Ok(transposed)
    }

    pub fn identity(n: usize) -> Matrix {
        let mut identity = Matrix::zeros(n,n);
        for i in 0..n {
            identity.set(i, i, 1.0);
        }
        identity
    }

    pub fn is_symmetric(&self, tol: f64) -> Result<bool, MatrixError> {
        if self.rows() != self.cols() {
            return Err(MatrixError::NotSquare);
        }

        for i in 0..self.rows() {
            for j in (i+1)..self.cols() {
                let diff = self.get(i, j)? - self.get(j, i)?;
                if diff.abs() > tol {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

/// 矩阵乘法：C = A * B
pub fn matmul(a: &Matrix, b: &Matrix) -> Result<Matrix, MatrixError> {
    if a.cols() != b.rows() {
        return Err(MatrixError::DimensionMismatch);
    }
    let mut c = Matrix::zeros(a.rows(), b.cols());
    for i in 0..a.rows() {
        for j in 0..b.cols() {
            let mut sum = 0.0;
            for k in 0..a.cols() {
                sum += a.get(i, k)? * b.get(k, j)?;
            }
            c.set(i, j, sum);
        }
    }
    Ok(c)
}

/// 矩阵–向量乘法：y = A * x
///
/// A: m × n
/// x: 长度为 n 的列向量
/// y: 长度为 m
///
/// 要求：
/// - 如果 A.cols() != x.len()，返回 Err(MatrixError::DimensionMismatch)
pub fn matvec(a: &Matrix, x: &[f64]) -> Result<Vec<f64>, MatrixError> {
    if a.cols() != x.len() {
        return Err(MatrixError::DimensionMismatch);
    }
    let mut y = Vec::with_capacity(a.rows());
    for i in 0..a.rows() {
        let mut sum = 0.0;
        for j in 0..a.cols() {
            sum += a.get(i, j)? * x[j];
        }
        y.push(sum);
    }
    Ok(y)
}