use lin_alg_lab::units::{
    LengthUnit, Quantity, QuantityError, add, convert, to_meters, unit_to_meter_factor,
};

fn main() -> Result<(), QuantityError> {
    // 假设你实现了 Quantity::new(value, unit)
    let d1 = Quantity::new(1.0, LengthUnit::Meter);
    let d2 = Quantity::new(100.0, LengthUnit::Centimeter);
    let d3 = Quantity::new(2.0, LengthUnit::Kilometer);

    println!("d1 = {:?}", d1);
    println!("d2 = {:?}", d2);
    println!("d3 = {:?}", d3);

    // 1. 单位 -> 米 的换算检查
    let d2_m = to_meters(&d2);
    let d3_m = to_meters(&d3);

    println!("d2 in meters = {:?}", d2_m);
    println!("d3 in meters = {:?}", d3_m);

    // 100 cm 应该等于 1 m
    assert!((d2_m.value() - 1.0).abs() < 1e-10);
    // 2 km 应该等于 2000 m
    assert!((d3_m.value() - 2000.0).abs() < 1e-10);

    // 2. 任意单位之间转换
    let d1_in_cm = convert(&d1, LengthUnit::Centimeter);
    let d3_in_cm = convert(&d3, LengthUnit::Centimeter);

    println!("d1 in cm = {:?}", d1_in_cm);
    println!("d3 in cm = {:?}", d3_in_cm);

    // 1 m = 100 cm
    assert!((d1_in_cm.value() - 100.0).abs() < 1e-10);
    // 2 km = 200000 cm
    assert!((d3_in_cm.value() - 200000.0).abs() < 1e-5);

    // 3. 相加：1 m + 100 cm = 2 m
    let sum = add(&d1, &d2)?;
    println!("d1 + d2 = {:?}", sum);
    assert!((sum.value() - 2.0).abs() < 1e-10);
    // sum 的单位应该是 d1 的单位（Meter）
    // 你可以在 Quantity 里提供一个 unit() 方法来检查

    println!("All unit tests passed.");
    Ok(())
}
