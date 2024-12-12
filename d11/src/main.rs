use std::collections::HashMap;
use rayon::prelude::*;


fn blink(stones: Vec<u64>) -> Vec<u64> {
    stones.iter().flat_map(|stone| {
        if *stone == 0 {
            return vec![1]
        } else {
            let stone_s = stone.to_string();
            if stone_s.len() % 2 == 0 {
                let half = stone_s.len() / 2;
                let (left, right) = stone_s.split_at(half);
                vec![left.parse().unwrap(), right.parse().unwrap()]
            } else {
                vec![*stone * 2024]
            }
        }
    }).collect()
}

// for part 2 im going to do recursion with caching. when running below, I'll start from the result of part 1
type BlinkCache = HashMap<(u64, u32), u64>;

fn even_len(stone: u64) -> bool {
    (((stone as f64).log10() as u64) + 1) % 2 == 0
}
fn split_even(stone: u64) -> (u64, u64) {
    let stone_s = stone.to_string();
    let half = stone_s.len() / 2;
    let (left, right) = stone_s.split_at(half);
    (left.parse().unwrap(), right.parse().unwrap())
}
fn blink_rec(stone: u64, blinks: u32, stop_at: u32, cache: &mut BlinkCache) -> u64 {
    if blinks >= stop_at {
        return 1;
    }
    if let Some(&cached) = cache.get(&(stone, blinks)) {
        return cached;
    }
    let result = if stone == 0 {
        blink_rec(1, blinks + 1, stop_at, cache)
    } else {
        if even_len(stone) {
            let (left, right) = split_even(stone);
            blink_rec(left, blinks + 1, stop_at, cache) + blink_rec(right, blinks + 1, stop_at, cache)
        } else {
            blink_rec(stone * 2024, blinks + 1, stop_at, cache)
        }
    };
    cache.insert((stone, blinks), result);
    result
}

fn main() {
    let input = include_str!("input.txt");
    let init_stones: Vec<u64> = input.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut result = init_stones.clone();
    for _ in 0..25 {
        result = blink(result);
    }
    println!("part 1: {:?}", result.len());

    // parallel map over part 1 stones. each thread will get it's own cache
    let result2: u64 = result.par_iter().map(|stone| {
        let mut cache = BlinkCache::new();
        blink_rec(*stone, 0, 50, &mut cache)
    }).reduce(|| 0, |x, y| x + y);
    println!("part 2: {:?}", result2);
}
