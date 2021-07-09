use super::board;

pub fn to_decimal(values: &[u8], base: u8) -> u64 {
    let mut result: u64 = 0;
    let mut base_multiplier: u64 = 1;

    for value in values {
        result += *value as u64 * base_multiplier;
        base_multiplier *= base as u64;
    }

    return result;
}

pub fn to_decimal_orientation(board: &[u8], rotate_fn: fn(u8, u8) -> u8) -> u64 {
    let mut result: u64 = 0;
    let mut base_multiplier: u64 = 1;

    for row in 0..board::SIZE {
        for col in 0..board::SIZE {
            let value = board[rotate_fn(row, col) as usize];
            result += value as u64 * base_multiplier;
            base_multiplier *= 16;
        }
    }

    return result;
}

#[test]
fn test_empty() {
    let values = [];
    let result = to_decimal(&values, 2);
    assert_eq!(result, 0);
}

#[test]
fn test_zero() {
    let values = [0];
    let result = to_decimal(&values, 2);
    assert_eq!(result, 0);
}

#[test]
fn test_zeros() {
    let values = [0, 0, 0, 0, 0];
    let result = to_decimal(&values, 2);
    assert_eq!(result, 0);
}

#[test]
fn test_717_binary() {
    let values = [1, 0, 1, 1, 0, 0, 1, 1, 0, 1];
    let result = to_decimal(&values, 2);
    assert_eq!(result, 717);
}

#[test]
fn test_717_octal() {
    let values = [5, 1, 3, 1];
    let result = to_decimal(&values, 8);
    assert_eq!(result, 717);
}

#[test]
fn test_717_hex() {
    let values = [13, 12, 2, 0, 0];
    let result = to_decimal(&values, 16);
    assert_eq!(result, 717);
}
