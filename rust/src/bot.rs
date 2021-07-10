use super::board;
use super::game;
use rand;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use std::collections::HashMap;

pub const N_EPISODES: i32 = 1000000;
pub const LEARNING_RATE: f32 = 0.1;
pub const DISCOUNT_VALUE: f32 = 0.9;
pub const EXPLORATION_RATE_DECAY: f32 = 0.02;

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

fn play_one_game(exploration_rate0: f32, states: &mut HashMap<(u64, u8), f32>) -> i32 {
    let mut board: [u8; board::SIZE2] = [0; board::SIZE2];
    game::place_new_piece(&mut board);
    let mut game_state = game::state(&mut board);
    let mut possible_dirs = game::get_possible_dirs(&board);
    let mut total_score = 0;
    let mut exploration_rate = exploration_rate0.clone();
    while possible_dirs.len() != 0 {
        let dir = choose_dir(game_state, &possible_dirs, exploration_rate, &states);

        let (mut score, next_game_state, next_possible_dirs) = game::play(&mut board, dir);
        possible_dirs = next_possible_dirs;
        total_score += score;

        let mut max_q = 0.0;
        if possible_dirs.len() == 0 {
            score = -20;
        } else {
            let next_best_dir = best_move(&possible_dirs, next_game_state, states);
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
        exploration_rate += 0.01;
    }

    // println!("{}, {}", count, exploration_rate);

    return total_score;
}

fn choose_dir(
    game_state: u64,
    possible_dir: &Vec<u8>,
    exploration_rate: f32,
    states: &HashMap<(u64, u8), f32>,
) -> u8 {
    let mut qs = Vec::<f32>::new();

    for dir in possible_dir {
        let state = (game_state, *dir);
        let option = states.get(&state);
        let mut q = 0.0;
        if option.is_some() {
            q += *option.unwrap();
        }
        qs.push(q);
    }

    // Formula does not work with a zero exploration_rate
    // When this happens just return the best dir
    // Also because of precision limitations
    //   doesn't work with exploration_rate less that 0.034
    if exploration_rate <= 0.034 {
        let mut max_i = 0;

        for (i, q) in qs.iter().enumerate() {
            if qs[max_i] < *q {
                max_i = i;
            }
        }

        return possible_dir[max_i];
    }

    let weights = if 1.0 <= exploration_rate {
        weights_formula(&qs, 1.0)
    } else {
        weights_formula(&qs, exploration_rate)
    };

    let mut rng = rand::thread_rng();
    let wi = WeightedIndex::new(&weights);
    if wi.is_err() {
        for w in &weights {
            print!("{} ", *w);
        }
        println!();
        println!();
        println!();
        println!();
        return possible_dir[0];
    }
    let dist = wi.unwrap();

    let dir = possible_dir[dist.sample(&mut rng)];

    return dir;
}

fn weights_formula(qs: &Vec<f32>, exploration_rate: f32) -> Vec<f32> {
    let mut weights = Vec::new();

    let exponential = 1.0 / exploration_rate;
    let mut min = 99999999999999999999999999999999999999.9;
    let mut powered_qs = Vec::new();

    for q in qs {
        let powered = if *q < 0.0 {
            -f32::powf(-*q, exponential)
        } else {
            f32::powf(*q, exponential)
        };

        assert!(powered != f32::NAN);
        powered_qs.push(powered);
        if powered < min {
            min = powered;
        }
    }

    let mut sum = 0.0;
    for powered in &powered_qs {
        sum += *powered - min;
    }

    let static_dist = exploration_rate / qs.len() as f32;

    // Formula does not work with a zero sum
    // When this happens just return the best standard distribution
    if sum == 0.0 {
        for _ in qs {
            weights.push(static_dist);
        }
        return weights;
    }

    for powered in &powered_qs {
        let weight = (1.0 - exploration_rate) * (*powered - min) / sum + static_dist;
        weights.push(weight);
    }

    return weights;
}

fn best_move(possible_dirs: &Vec<u8>, state: u64, states: &HashMap<(u64, u8), f32>) -> u8 {
    let mut dir = possible_dirs[0];
    let mut best_q = -100.0;

    for possible_dir in possible_dirs {
        let option = states.get(&(state, *possible_dir));
        let mut q = 0.0;
        if option.is_some() {
            q = *option.unwrap();
        }
        if best_q < q {
            best_q = q;
            dir = *possible_dir;
        }
    }

    return dir;
}
