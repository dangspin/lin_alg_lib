use lin_alg_lab::matrix_dense::{Matrix, MatrixError, matmul, matvec};

fn main() -> Result<(), MatrixError> {
    // 构造一个 2×3 矩阵 A：
    //
    // [ 1  2  3 ]
    // [ 4  5  6 ]
    //
    // 我们约定按行优先存储：
    // data = [1,2,3, 4,5,6]
    let a = Matrix::new(
        2,
        3,
        vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ],
    )?;

    // 构造一个 3×2 矩阵 B：
    //
    // [ 7   8 ]
    // [ 9  10 ]
    // [11  12 ]
    //
    // data = [7,8, 9,10, 11,12]
    let b = Matrix::new(
        3,
        2,
        vec![
             7.0,  8.0,
             9.0, 10.0,
            11.0, 12.0,
        ],
    )?;

    // C = A * B 应该是 2×2 矩阵：
    //
    // [  58   64 ]   = [1,2,3]·[7,9,11], [1,2,3]·[8,10,12]
    // [ 139  154 ]     [4,5,6]·[7,9,11], [4,5,6]·[8,10,12]
    let c = matmul(&a, &b)?;
    println!("C = A * B = {:?}", c);

    assert_eq!(c.rows(), 2);
    assert_eq!(c.cols(), 2);
    assert!((c.get(0, 0)? - 58.0).abs() < 1e-10);
    assert!((c.get(0, 1)? - 64.0).abs() < 1e-10);
    assert!((c.get(1, 0)? - 139.0).abs() < 1e-10);
    assert!((c.get(1, 1)? - 154.0).abs() < 1e-10);

    // 再做一个矩阵–向量乘法：
    //
    // 取 x = [1, 2, 3]^T，则 A * x =
    // [ 1*1 + 2*2 + 3*3 ] = 14
    // [ 4*1 + 5*2 + 6*3 ] = 32
    let x = vec![1.0, 2.0, 3.0];
    let y = matvec(&a, &x)?;
    println!("y = A * x = {:?}", y);

    assert_eq!(y.len(), 2);
    assert!((y[0] - 14.0).abs() < 1e-10);
    assert!((y[1] - 32.0).abs() < 1e-10);

    println!("Matrix dense demo passed.");
    Ok(())
}