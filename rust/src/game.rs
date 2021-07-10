use super::base;
use super::board;
use super::board::{Board, OrientationFn, SIZE, SIZE2};
use rand;

// Good
pub fn get_possible_dirs(board: &Board) -> Vec<u8> {
    let mut possible_dirs = Vec::new();

    for dir in 0..4 {
        let orientation_fn = dir_to_orientation(dir);
        if orientation_has_move(board, orientation_fn) {
            possible_dirs.push(dir);
        }
    }

    return possible_dirs;
}

// Good
fn orientation_has_move(board: &Board, orientation_fn: OrientationFn) -> bool {
    for row in 0..SIZE {
        if row_has_move(board, orientation_fn, row) {
            return true;
        }
    }

    return false;
}

// Good
fn row_has_move(board: &Board, orientation_fn: OrientationFn, row: usize) -> bool {
    let mut has_zero = false;
    let mut prev = 255;

    for col in 0..SIZE {
        let piece = board[orientation_fn(row, col)];

        if piece == 0 {
            has_zero = true;
        } else if has_zero || prev == piece {
            return true;
        }

        prev = piece;
    }

    return false;
}

pub fn move_board(board: &mut Board, dir: u8) -> i32 {
    let mut score = 0;
    let orientation_fn = dir_to_orientation(dir);

    for row in 0..SIZE {
        score += move_row(board, orientation_fn, row);
    }

    return score;
}

fn move_row(board: &mut Board, orientation_fn: OrientationFn, row: usize) -> i32 {
    let mut score = 0;
    let mut new_col = 0;
    let mut new_position = orientation_fn(row, new_col);

    for col in 0..SIZE {
        let position = orientation_fn(row, col);
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
                new_position = orientation_fn(row, new_col);
            } else {
                new_col += 1;
                new_position = orientation_fn(row, new_col);
                board[new_position] = piece;
            }
        }
    }
    return score;
}

fn dir_to_orientation(dir: u8) -> OrientationFn {
    match dir {
        0 => board::orientation0,
        1 => board::orientation3,
        2 => board::orientation2,
        3 => board::orientation1,
        _ => board::orientation0,
    }
}

// why return
pub fn place_new_piece(board: &mut Board) -> bool {
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
pub fn state(board: &mut Board) -> u64 {
    let orientations = [
        board::orientation1,
        board::orientation2,
        board::orientation3,
        board::orientation0_t,
        board::orientation1_t,
        board::orientation2_t,
        board::orientation3_t,
    ];

    let mut chosen_orientation: OrientationFn = board::orientation0;
    let mut state = base::to_decimal_orientation(board, chosen_orientation);

    for orientation in orientations {
        let s2 = base::to_decimal_orientation(board, orientation);
        if state < s2 {
            chosen_orientation = orientation;
            state = s2;
        }
    }

    let mut new_board: Board = [0; SIZE2];
    let mut i = 0;

    for row in 0..SIZE {
        for col in 0..SIZE {
            let pos = chosen_orientation(row, col);
            let piece = board[pos];
            new_board[i] = piece;
            i += 1;
        }
    }

    *board = new_board;
    return state;
}

pub fn play(board: &mut Board, dir: u8) -> (i32, u64, Vec<u8>) {
    let score = move_board(board, dir);
    place_new_piece(board);
    let state = state(board);
    let possible_dirs = get_possible_dirs(board);

    return (score, state, possible_dirs);
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
fn test_possible_dirs() {
    let board = [3, 2, 1, 2];
    let result = get_possible_dirs(&board);

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
