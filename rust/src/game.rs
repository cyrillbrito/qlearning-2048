use super::base;
use super::board;
use rand;

// TODO This can be improved!
// Instead of trying every move run a custom loop to check for possible moves
pub fn get_possible_moves(board: &[u8; board::SIZE2]) -> Vec<u8> {
    let mut possible_moves = Vec::new();

    for dir in 0..4 {
        let rotate_fn = dir_to_rotation(dir);
        if orientation_has_move(board, rotate_fn) {
            possible_moves.push(dir);
        }
    }

    return possible_moves;
}

fn orientation_has_move(board: &[u8], rotate_fn: fn(u8, u8) -> u8) -> bool {
    for row in 0..board::SIZE {
        if row_as_move(board, rotate_fn, row) {
            return true;
        }
    }

    return false;
}

fn row_as_move(board: &[u8], rotate_fn: fn(u8, u8) -> u8, row: u8) -> bool {
    let mut has_zero = false;
    let mut prev = 255;

    for col in 0..board::SIZE {
        let piece = board[rotate_fn(row, col) as usize];

        if piece == 0 {
            has_zero = true;
        } else if has_zero || prev == piece {
            return true;
        }

        prev = piece;
    }

    return false;
}

pub fn move_board(board: &mut [u8], dir: u8) -> i32 {
    let mut score = 0;
    let rotate_fn = dir_to_rotation(dir);

    for row in 0..board::SIZE {
        score += move_row(board, rotate_fn, row);
    }

    return score;
}

fn move_row(board: &mut [u8], rotate_fn: fn(u8, u8) -> u8, row: u8) -> i32 {
    let mut score = 0;
    let mut new_col = 0;
    let mut new_position = rotate_fn(row, new_col) as usize;

    for col in 0..board::SIZE {
        let position = rotate_fn(row, col) as usize;
        let piece = board[position];

        if piece != 0 {
            board[position] = 0;
            if board[new_position] == 0 {
                board[new_position] = piece;
            } else if board[new_position] == piece {
                // score += 2 * (piece as i32).pow(2);
                score += 1;
                board[new_position] += 1;
                new_col += 1;
                new_position = rotate_fn(row, new_col) as usize;
            } else {
                new_col += 1;
                new_position = rotate_fn(row, new_col) as usize;
                board[new_position] = piece;
            }
        }
    }
    return score;
}

fn dir_to_rotation(dir: u8) -> fn(u8, u8) -> u8 {
    if dir == 0 {
        return board::rotate0;
    } else if dir == 1 {
        return board::rotate3;
    } else if dir == 2 {
        return board::rotate2;
    } else {
        return board::rotate1;
    }
}

pub fn place_new_piece(board: &mut [u8]) -> bool {
    let mut zero_positions = Vec::new();
    for (i, piece) in board.iter().enumerate() {
        if *piece == 0 {
            zero_positions.push(i);
        }
    }

    let len = zero_positions.len();
    if len == 0 {
        return false;
    }

    let r: f32 = rand::random();
    let choice = (r * len as f32).floor() as usize;
    let position = zero_positions[choice];
    board[position] = 1;

    return true;
}

// This can be improved, curr it tries every orientation to see the smallest
// It could iterate piece by piece for the smallest
pub fn state(board: &mut [u8; board::SIZE2]) -> u64 {
    let orientations = [
        board::rotate1,
        board::rotate2,
        board::rotate3,
        board::rotate0_t,
        board::rotate1_t,
        board::rotate2_t,
        board::rotate3_t,
    ];

    let mut chosen_orientation: fn(u8, u8) -> u8 = board::rotate0;
    let mut state = base::to_decimal_orientation(board, chosen_orientation);

    for orientation in orientations {
        let s2 = base::to_decimal_orientation(board, orientation);
        if state < s2 {
            chosen_orientation = orientation;
            state = s2;
        }
    }

    let mut new_board: [u8; board::SIZE2] = [0; board::SIZE2];
    let mut i = 0;

    for row in 0..board::SIZE {
        for col in 0..board::SIZE {
            let pos = chosen_orientation(row, col) as usize;
            let piece = board[pos];
            new_board[i] = piece;
            i += 1;
        }
    }

    *board = new_board;
    return state;
}

#[test]
fn test_place_new() {
    let mut board = [0, 0, 1, 2];
    let result = place_new_piece(&mut board);

    assert_eq!(true, result);
    assert_eq!(
        true,
        (board[0] == 1 && board[1] == 0) || (board[0] == 0 && board[1] == 1)
    );
}

#[test]
fn test_place_new_error() {
    let mut board = [3, 2, 1, 2];
    let result = place_new_piece(&mut board);

    assert_eq!(false, result);
}

#[test]
fn test_possible_moves() {
    let board = [3, 2, 1, 2];
    let result = get_possible_moves(&board);

    assert_eq!(0, result.len());
}

#[test]
fn test_state() {
    let og_state = state(&[0, 1, 2, 3]);
    let alternatives = [
        [2, 0, 3, 1],
        [3, 2, 1, 0],
        [1, 3, 0, 2],
        [0, 2, 1, 3],
        [1, 0, 3, 2],
        [3, 1, 2, 0],
        [2, 3, 0, 1],
    ];

    for alt in alternatives {
        assert_eq!(og_state, state(&alt));
    }
}

#[test]
fn test_state_diff() {
    let og_state = state(&[0, 1, 2, 3]);
    let alternatives = [
        [3, 2, 0, 1],
        [0, 3, 1, 2],
        [1, 3, 1, 2],
        [1, 3, 2, 4],
        [0, 3, 0, 1],
    ];

    for alt in alternatives {
        assert_ne!(og_state, state(&alt));
    }
}
