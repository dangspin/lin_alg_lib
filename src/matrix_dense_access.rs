use crate::matrix_dense::{Matrix, MatrixError};

/// 提示：
/// - 用已有的 `rows()`, `cols()` 判断边界
/// - 如果越界，返回 `MatrixError::DimensionMismatch`
/// - 如有需要，可以用内部的数据访问 + `ok_or` + `?`，把 Option 转成 Result
pub fn safe_get(a: &Matrix, row: usize, col: usize) -> Result<f64, MatrixError> {
    if row >= a.rows() || col >= a.cols() {
        return Err(MatrixError::DimensionMismatch);
    }

    Ok(a.get(row, col)?)
}

/// 提示：
/// - 尝试复用上面的 `safe_get`，不要重复写越界检查逻辑
/// - 注意：这里是批量访问，任何一个位置出错都应该让整个函数返回 Err
pub fn safe_get_row(a: &Matrix, row: usize) -> Result<Vec<f64>, MatrixError> {
    let mut row_data = Vec::with_capacity(a.cols());
    
    for col in 0..a.cols() {
        row_data.push(safe_get(a, row, col)?);
    }

    Ok(row_data)
}

/// 提示：
/// - 这里可以练习 `ok_or_else`：
///   * 不要提前构造错误，而是用闭包延迟构造
/// - 当提供的 indices 里有某个 (row, col) 越界时，同样返回 DimensionMismatch
pub fn safe_batch_get(
    a: &Matrix,
    indices: &[(usize, usize)],
) -> Result<Vec<f64>, MatrixError> {
    let mut batch_data = Vec::with_capacity(indices.len());
    
    for &(row, col) in indices {
        batch_data.push(safe_get(a, row, col)?);
    }
    
    Ok(batch_data)
}