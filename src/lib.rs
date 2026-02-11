// lin_alg_lab/src/lib.rs
pub mod expr;
pub mod iter_utils;
pub mod matrix;
pub mod matrix_dense;
pub mod numeric;
pub mod path;
pub mod units;
pub mod vector;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
