// 你需要自己定义一个结构体 WindowIter<'a>：
// 建议字段：
// - slice: &'a [f64]      // 原始数据切片
// - window_size: usize    // 窗口大小
// - pos: usize            // 当前窗口起始位置（0, 1, 2, ...）
#[derive(Debug, Clone, Copy)]
pub struct WindowIter<'a> {
    slice: &'a [f64],
    window_size: usize,
    pos: usize,
}

// 在这里自己定义 WindowIter<'a> ...

impl<'a> WindowIter<'a> {
    /// 创建一个新的 WindowIter。
    ///
    /// 约定：
    /// - 如果 window_size == 0，可以直接返回一个“空迭代器”（后续 next() 永远返回 None）。
    /// - 如果 window_size > slice.len()，同样可以看作没有任何窗口。
    pub fn new(slice: &'a [f64], window_size: usize) -> Self {
        if window_size == 0 || window_size > slice.len() {
            return Self {
                slice,
                window_size,
                pos: 0,
            }
        } else {
            Self {
                slice,
                window_size,
                pos: 0,
            }
        }
    }
}

/// 实现 Iterator，用于按顺序产生所有窗口：
///
/// 对 data = [1, 2, 3, 4], window_size = 3，窗口应该依次是：
/// - &[1, 2, 3]
/// - &[2, 3, 4]
impl<'a> Iterator for WindowIter<'a> {
    type Item = &'a [f64];

    fn next(&mut self) -> Option<Self::Item> {
        if self.window_size == 0 || self.pos + self.window_size > self.slice.len() {
            return None;
        } else {
            let window = &self.slice[self.pos..self.pos + self.window_size];
            self.pos += 1;
            return Some(window);
        }
    }
}

/// 方便用户使用的构造函数：
///
/// windows(&data, k) 相当于 WindowIter::new(&data, k)
pub fn windows(data: &[f64], window_size: usize) -> WindowIter<'_> {
    WindowIter::new(data, window_size)
}

/// 基于滑动窗口计算简单移动平均：
///
/// 输入：
/// - data: &[f64]
/// - window_size: usize
///
/// 输出：
/// - Vec<f64>，每个元素是一个窗口里所有元素的平均值。
///
/// 例如：
/// - data = [1.0, 2.0, 3.0, 4.0], window_size = 3
///   窗口： [1,2,3], [2,3,4]
///   输出： [2.0, 3.0]
pub fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    let mut result = Vec::new();

    for window in windows(data, window_size) {
        let avg = window.iter().sum::<f64>() / window_size as f64;
        result.push(avg);
    }

    result
}

/// 窗口方差（population variance）
///
/// 对每个窗口 w：
/// - mean = 平均值
/// - mean_sq = 每个元素平方后的平均值
/// - var = mean_sq - mean * mean
///
/// 建议实现思路（供你参考，不要原样照抄）：
/// - 先用 `moving_mean(data, window_size)` 得到每个窗口的均值序列 m[i]
/// - 构造一份平方后的数据：xsq[j] = data[j]^2
/// - 再用 moving_mean 计算 xsq 的均值，得到 msq[i]
/// - var[i] = msq[i] - m[i] * m[i]
///
/// 边界条件：
/// - window_size == 0 或 window_size > data.len() 时，返回空 Vec。
pub fn moving_variance(data: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || window_size > data.len() {
        return Vec::new();
    }

    let data_sq = data.iter().map(|x| (*x) * (*x)).collect::<Vec<f64>>();
    let mean = moving_average(data, window_size);
    let mean_sq = moving_average(&data_sq, window_size);
    let var = mean_sq.iter().zip(mean.iter()).map(|(msq, mean)| *msq - *mean * *mean).collect::<Vec<f64>>();
    var
}

/// 窗口标准差：std = sqrt(var)
///
/// 可以基于 moving_variance 实现：对每个 var[i] 开方。
pub fn moving_stddev(data: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || window_size > data.len() {
        return Vec::new();
    }

    let var = moving_variance(data, window_size);
    let stddev = var.iter().map(|x| x.sqrt()).collect::<Vec<f64>>();
    stddev
}

/// 指数移动平均（Exponential Moving Average, EMA）
///
/// 参数：
/// - data: 输入数据
/// - alpha: 平滑因子，通常在 (0, 1] 之间
///
/// 约定：
/// - 如果 data 为空，返回空 Vec；
/// - 如果 alpha <= 0 或 alpha > 1，你可以选择返回空 Vec 或者 panic，
///   自己挑一个策略并在实现里用注释说明。
///
/// 定义：
/// - ema[0] = data[0]
/// - ema[i] = alpha * data[i] + (1 - alpha) * ema[i-1]
pub fn exponential_moving_average(data: &[f64], alpha: f64) -> Vec<f64> {
    if data.is_empty() || alpha <= 0.0 || alpha > 1.0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    result.push(data[0]);
    for i in 1..data.len() {
        let ema = alpha * data[i] + (1.0 - alpha) * result[i - 1];
        result.push(ema);
    }
    result
}

/// 基于滑动窗口的简单异常检测（z-score 方法）。
///
pub fn detect_1d_outliers_zscore(
    data: &[f64],
    window_size: usize,
    threshold: f64,
) -> Vec<bool> {
    let n = data.len();
    if window_size == 0 || window_size > n {
        return vec![false; n];
    }

    if n < window_size + 1 {
        return vec![false; n];
    }

    let history_end = n - 1;
    let means = moving_average(&data[..history_end], window_size);
    let stddevs = moving_stddev(&data[..history_end], window_size);

    let mut result = vec![false; n];

    for i in 0..means.len() {
        let m = means[i];
        let s = stddevs[i];
        let idx = i + window_size;

        if s.abs() < 1e-12 {
            if (data[idx] - m).abs() > 1e-12 {
                result[idx] = true;
            }
        } else {
            let z = (data[idx] - m) / s;
            if z.abs() > threshold {
                result[idx] = true;
            }
        }
    }

    result
}