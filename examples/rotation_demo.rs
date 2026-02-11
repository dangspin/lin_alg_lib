use lin_alg_lab::matrix::Matrix2x2;
use lin_alg_lab::vector::Vector2;

fn main() {
    let v = Vector2::new(1.0, 0.0);

    // 旋转 90 度（π/2）
    let rot90 = Matrix2x2::rotation(std::f64::consts::FRAC_PI_2);
    let v_rot90 = rot90 * v;
    let v_rot90_ref = rot90 * &v;

    println!("v           = {:?}", v);
    println!("rot90 * v   = {:?}", v_rot90);
    println!("rot90 * &v  = {:?}", v_rot90_ref);

    // (1, 0) 旋转 90° 应该接近 (0, 1)
    assert!(v_rot90.x().abs() < 1e-10);
    assert!((v_rot90.y() - 1.0).abs() < 1e-10);

    // 旋转 180 度（π）
    let rot180 = Matrix2x2::rotation(std::f64::consts::PI);
    let v_rot180 = rot180 * v;

    println!("rot180 * v  = {:?}", v_rot180);

    // (1, 0) 旋转 180° 应该接近 (-1, 0)
    assert!((v_rot180.x() + 1.0).abs() < 1e-10);
    assert!(v_rot180.y().abs() < 1e-10);

    println!("All rotation checks passed.");
}
