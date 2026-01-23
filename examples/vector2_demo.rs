use lin_alg_lab::vector::{
    dot, norm, project_onto, distance, Vector2,
};

fn main() {
    let v1 = Vector2::new(3.0, 4.0);
    let v2 = Vector2::new(-4.0, 3.0);

    // 1. 基本运算
    let sum = v1 + v2;
    let diff = v1 - v2;
    let neg = -v1;
    let scaled1 = v1 * 2.0;
    let scaled2 = 2.0 * v1;

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v1 + v2 = {:?}", sum);
    println!("v1 - v2 = {:?}", diff);
    println!("-v1 = {:?}", neg);
    println!("v1 * 2.0 = {:?}", scaled1);
    println!("2.0 * v1 = {:?}", scaled2);

    // 2. 长度 / 长度平方
    println!("‖v1‖^2 = {}", v1.length_sq());
    println!("‖v1‖ = {}", norm(&v1));

    // 3. 归一化
    match v1.normalize() {
        Some(unit) => {
            println!("v1 normalized = {:?}", unit);
            println!("‖unit‖ = {}", norm(&unit));
        }
        None => {
            println!("v1 is too close to zero to normalize");
        }
    }

    let zero = Vector2::new(0.0, 0.0);
    match zero.normalize() {
        Some(unit) => println!("zero normalized (unexpected) = {:?}", unit),
        None => println!("zero.normalize() correctly returned None"),
    }

    // 4. 距离
    let dist = distance(&v1, &v2);
    println!("distance(v1, v2) = {}", dist);

    // 5. 继续用投影检验
    let proj = project_onto(&v1, &v2);
    println!("proj_v2(v1) = {:?}", proj);
}