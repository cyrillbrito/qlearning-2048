pub const SIZE: usize = 3;
pub const SIZE2: usize = 9;

pub type OrientationFn = fn(usize, usize) -> usize;
pub type Board = [u8; SIZE2];

// Orientation helper functions

fn low_one_by_one(x: usize) -> usize {
    return x;
}

fn low_size_by_size(x: usize) -> usize {
    return x * SIZE;
}

fn high_one_by_one(x: usize) -> usize {
    return SIZE - x - 1;
}

fn high_size_by_size(x: usize) -> usize {
    return SIZE * (SIZE - x - 1);
}

// Orientation functions

pub fn orientation0(row: usize, col: usize) -> usize {
    return low_size_by_size(row) + low_one_by_one(col);
}

pub fn orientation1(row: usize, col: usize) -> usize {
    return low_one_by_one(row) + high_size_by_size(col);
}

pub fn orientation2(row: usize, col: usize) -> usize {
    return high_size_by_size(row) + high_one_by_one(col);
}

pub fn orientation3(row: usize, col: usize) -> usize {
    return high_one_by_one(row) + low_size_by_size(col);
}

pub fn orientation0_t(row: usize, col: usize) -> usize {
    return low_size_by_size(row) + high_one_by_one(col);
}

pub fn orientation1_t(row: usize, col: usize) -> usize {
    return high_one_by_one(row) + high_size_by_size(col);
}

pub fn orientation2_t(row: usize, col: usize) -> usize {
    return high_size_by_size(row) + low_one_by_one(col);
}

pub fn orientation3_t(row: usize, col: usize) -> usize {
    return low_one_by_one(row) + low_size_by_size(col);
}

#[allow(dead_code)]
pub fn print(board: &Board) {
    println!("-----");
    for (i, piece) in board.iter().enumerate() {
        if (i + 1) % SIZE == 0 {
            println!("{}", piece);
        } else {
            print!("{} ", piece);
        }
    }
}
