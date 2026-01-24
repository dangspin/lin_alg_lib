use lin_alg_lab::iter_utils::{windows, moving_average};

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    println!("data = {:?}", data);

    // 1. 直接使用 windows 迭代器
    let mut iter = windows(&data, 3);
    while let Some(w) = iter.next() {
        println!("window: {:?}", w);
    }

    // 2. 使用 moving_average 计算移动平均
    let ma3 = moving_average(&data, 3);
    println!("moving average (k=3) = {:?}", ma3);

    // 简单检查
    // 窗口：[1,2,3], [2,3,4], [3,4,5]
    // 平均：2.0, 3.0, 4.0
    assert_eq!(ma3.len(), 3);
    assert!((ma3[0] - 2.0).abs() < 1e-10);
    assert!((ma3[1] - 3.0).abs() < 1e-10);
    assert!((ma3[2] - 4.0).abs() < 1e-10);

    println!("All moving average checks passed.");
}