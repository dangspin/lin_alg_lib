use lin_alg_lab::matrix_dense::{Matrix, MatrixError, matmul, matvec};

fn max_abs_diff(a: &Matrix, b: &Matrix) -> Result<f64, MatrixError> {
    if a.rows() != b.rows() || a.cols() != b.cols() {
        return Err(MatrixError::DimensionMismatch);
    }

    let mut m = 0.0;
    for i in 0..a.rows() {
        for j in 0..a.cols() {
            let d = (a.get(i, j)? - b.get(i, j)?).abs();
            if d > m {
                m = d;
            }
        }
    }
    Ok(m)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 构造矩阵 A:
    // [ 4  3 ]
    // [ 6  3 ]
    let a = Matrix::new(2, 2, vec![4.0, 3.0, 6.0, 3.0])?;

    println!("Original Matrix A:\n{:?}", a);

    // 1. 执行分解
    let (l, u) = a.lu_decomposition()?;
    println!("L:\n{:?}", l);
    println!("U:\n{:?}", u);

    // 验证 A = LU
    let lu = matmul(&l, &u)?;
    let diff = max_abs_diff(&a, &lu)?;
    println!("max |A - L*U| = {}", diff);
    assert!(diff < 1e-10);

    // 2. 解方程组 Ax = b, 其中 b = [7, 9]^T
    // 对应方程：
    // 4x + 3y = 7
    // 6x + 3y = 9
    // 解应为 x=1, y=1
    let b = vec![7.0, 9.0];
    let x = Matrix::solve_lu(&l, &u, &b)?;

    println!("Solution x: {:?}", x);

    assert!((x[0] - 1.0).abs() < 1e-10);
    assert!((x[1] - 1.0).abs() < 1e-10);

    let ax = matvec(&a, &x)?;
    let r0 = ax[0] - b[0];
    let r1 = ax[1] - b[1];
    let r_inf = r0.abs().max(r1.abs());
    println!("||Ax - b||_inf = {}", r_inf);
    assert!(r_inf < 1e-10);

    println!("LU decomposition demo passed!");
    Ok(())
}
