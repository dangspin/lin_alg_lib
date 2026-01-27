/// 练习目标：
/// - 定义一个长度单位枚举 LengthUnit（比如 Meter, Centimeter, Kilometer）
/// - 定义一个带单位的物理量 Quantity（比如 1.0 m, 25.0 cm）
/// - 支持：
///   - 不同单位之间的换算（convert）
///   - 同维度的加法（add）
///   - 使用 Result + 自定义错误类型 QuantityError

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Meter,
    Centimeter,
    Kilometer,
}

#[derive(Debug, Clone)]
pub struct Quantity {
    value: f64,
    unit: LengthUnit,
}

impl Quantity {
    pub fn new(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(Debug)]
pub enum QuantityError {
    IncompatibleUnits,
}

/// 将某个长度单位转换成“对应到基准单位米(m)的倍率”。
/// 提示：这里非常适合用 match。
pub fn unit_to_meter_factor(unit: LengthUnit) -> f64 {
    match &unit {
        LengthUnit::Meter => 1.0,
        LengthUnit::Centimeter => 0.01,
        LengthUnit::Kilometer => 1000.0,
    }
}

/// 把一个带单位的 Quantity 换算成“以米为单位”的 Quantity。
pub fn to_meters(q: &Quantity) -> Quantity {
    let factor = unit_to_meter_factor(q.unit);
    Quantity::new(q.value * factor, LengthUnit::Meter)
}

/// 将一个 Quantity 换算成指定的目标单位。
///
/// 例如：
/// - 1.0 m 转成 cm = 100.0 cm
/// - 2.0 km 转成 m = 2000.0 m
///
/// 提示：
/// - 可以先用 to_meters 把输入转换成米；
/// - 再根据目标单位的 factor 计算新的 value。
pub fn convert(q: &Quantity, target_unit: LengthUnit) -> Quantity {
    let tmp_q = to_meters(q);
    let value = match target_unit {
        LengthUnit::Meter => tmp_q.value * 1.0,
        LengthUnit::Centimeter => tmp_q.value * 100.0,
        LengthUnit::Kilometer => tmp_q.value * 0.001,
    };
    Quantity::new(value, target_unit)
}

/// 尝试把两个 Quantity 相加，返回一个新的 Quantity。
///
/// 约定：
/// - 返回值的单位使用第一个参数的单位（a.unit）；
/// - 实现时可以：
///   - 先把 b 换算到 a 的单位下，
///   - 然后相加数值部分。
///
/// 目前只有长度维度，可以先假设都是长度；
/// 后续如果你扩展出 TimeUnit / MassUnit 等，可以在这里检查维度，
/// 不同维度时返回 Err(QuantityError::IncompatibleUnits)。
pub fn add(a: &Quantity, b: &Quantity) -> Result<Quantity, QuantityError> {
    if a.unit != b.unit {
        return Err(QuantityError::IncompatibleUnits);
    }
    let b_in_a_unit = convert(b, a.unit);
    Ok(Quantity::new(a.value + b_in_a_unit.value, a.unit))
}