use super::base;
use super::board;
use super::game;
use rand;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;

pub const N_EPISODES: i32 = 400000;
pub const LEARNING_RATE: f32 = 0.1;
pub const DISCOUNT_VALUE: f32 = 0.8;
pub const EXPLORATION_RATE_DECAY: f32 = 0.015;

pub fn go() {
    let mut current_percent = 0;
    let mut exploration_rate = 1.0;
    let mut score_sum = 0;
    let mut states = HashMap::new();

    for i in 0..N_EPISODES {
        score_sum += play_one_game(exploration_rate, &mut states);

        if current_percent < i * 100 / N_EPISODES {
            exploration_rate -= EXPLORATION_RATE_DECAY;
            current_percent += 1;
            let avg = score_sum / (N_EPISODES / 100);
            score_sum = 0;
            println!(
                "perc: {}%    step: {}    explorationRate: {}    AverageScore: {}",
                current_percent, i, exploration_rate, avg
            );
        }
    }

    println!("TOTAL Number of states {}", states.len());
    // for _i in 0..10 {
    //     score_sum += play_one_game(0.0, &mut states);
    // }
}

fn play_one_game(exploration_rate: f32, states: &mut HashMap<(u64, u8), f32>) -> i32 {
    let mut board: [u8; board::SIZE2] = [0; board::SIZE2];
    game::place_new_piece(&mut board);
    let mut possible_moves = game::get_possible_moves(&board);
    // let mut game_state = game::state(&board);
    let mut game_state = base::to_decimal(&board, 16);
    let mut total_score = 0;

    while possible_moves.len() != 0 {
        let dir = choose_dir(game_state, &possible_moves, exploration_rate, &states);

        let mut score = game::move_board(&mut board, dir);
        game::place_new_piece(&mut board);
        if exploration_rate == 0.0 {
            // board::print(&board);
        }
        total_score += score;

        // let next_game_state = game::state(&board);
        let next_game_state = base::to_decimal(&board, 16);
        possible_moves = game::get_possible_moves(&board);

        let mut max_q = 0.0;
        if possible_moves.len() == 0 {
            score = -20;
        } else {
            let next_best_dir = best_move(&possible_moves, next_game_state, states);
            let option = states.get(&(next_game_state, next_best_dir));
            if option.is_some() {
                max_q = *option.unwrap();
            }
        }

        let mut q = 0.0;
        let option2 = states.get(&(game_state, dir));
        if option2.is_some() {
            q = *option2.unwrap();
        }

        states.insert(
            (game_state, dir),
            (1.0 - LEARNING_RATE) * q + LEARNING_RATE * (score as f32 + DISCOUNT_VALUE * max_q),
        );

        game_state = next_game_state;
    }

    return total_score;
}

fn choose_dir(
    game_state: u64,
    possible_dir: &Vec<u8>,
    exploration_rate: f32,
    states: &HashMap<(u64, u8), f32>,
) -> u8 {
    let mut weights = Vec::<f32>::new();

    for dir in possible_dir {
        let state = (game_state, *dir);
        let option = states.get(&state);
        let mut q = 1.0;
        if option.is_some() {
            q += *option.unwrap();
            // println!("{}", q);
        }

        weights.push(q * (1.0 - exploration_rate));
    }

    let mut rng = rand::thread_rng();
    let wi = WeightedIndex::new(&weights);

    if wi.is_ok() {
        let dist = wi.unwrap();
        let dir = possible_dir[dist.sample(&mut rng)];
        return dir;
    } else {
        return *possible_dir.choose(&mut rng).unwrap();
    }
}

fn best_move(possible_moves: &Vec<u8>, state: u64, states: &HashMap<(u64, u8), f32>) -> u8 {
    let mut dir = possible_moves[0];
    let mut best_q = -100.0;

    for possible_move in possible_moves {
        let option = states.get(&(state, *possible_move));
        let mut q = 0.0;
        if option.is_some() {
            q = *option.unwrap();
        }
        if best_q < q {
            best_q = q;
            dir = *possible_move;
        }
    }

    return dir;
}
