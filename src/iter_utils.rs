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