pub const SIZE: u8 = 3;
pub const SIZE2: usize = 9;

// Rotate helper functions

fn low_one_by_one(x: u8) -> u8 {
    return x;
}

fn low_size_by_size(x: u8) -> u8 {
    return x * SIZE;
}

fn high_one_by_one(x: u8) -> u8 {
    return SIZE - x - 1;
}

fn high_size_by_size(x: u8) -> u8 {
    return SIZE * (SIZE - x - 1);
}

// Rotate functions

pub fn rotate0(row: u8, col: u8) -> u8 {
    return low_size_by_size(row) + low_one_by_one(col);
}

pub fn rotate1(row: u8, col: u8) -> u8 {
    return low_one_by_one(row) + high_size_by_size(col);
}

pub fn rotate2(row: u8, col: u8) -> u8 {
    return high_size_by_size(row) + high_one_by_one(col);
}

pub fn rotate3(row: u8, col: u8) -> u8 {
    return high_one_by_one(row) + low_size_by_size(col);
}

pub fn rotate0_t(row: u8, col: u8) -> u8 {
    return low_size_by_size(row) + high_one_by_one(col);
}

pub fn rotate1_t(row: u8, col: u8) -> u8 {
    return high_one_by_one(row) + high_size_by_size(col);
}

pub fn rotate2_t(row: u8, col: u8) -> u8 {
    return high_size_by_size(row) + low_one_by_one(col);
}

pub fn rotate3_t(row: u8, col: u8) -> u8 {
    return low_one_by_one(row) + low_size_by_size(col);
}

pub fn print(board: &[u8; SIZE2]) {
    println!("-----");
    for (i, piece) in board.iter().enumerate() {
        if (i + 1) % SIZE as usize == 0 {
            println!("{}", piece);
        } else {
            print!("{} ", piece);
        }
    }
}
