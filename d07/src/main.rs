use itertools::{Itertools, MultiProduct};

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
    Combine,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Combine => {
                (a.to_string() + &b.to_string()).parse().unwrap()
            }
        }
    }
}

// via https://stackoverflow.com/a/68231315
/// Rust version of Python's itertools.product().
/// It returns the cartesian product of the input iterables, and it is
/// semantically equivalent to `repeat` nested for loops.
///
/// # Arguments
///
/// * `it` - An iterator over a cloneable data structure
/// * `repeat` - Number of repetitions of the given iterator
pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
  where
    I: Iterator + Clone,
    I::Item: Clone {
  std::iter::repeat(it)
    .take(repeat)
    .multi_cartesian_product()
}


fn test_valid(goal: u64, nums: Vec<u64>, ops: Vec<Op>) -> bool {
    let op_configs = product_repeat(ops.into_iter(), nums.len() - 1);
    for config in op_configs {
        let mut acc = nums[0];
        for (op, num) in config.iter().zip(nums.iter().skip(1)) {
            acc = op.apply(acc, *num);
        }
        if acc == goal {
            return true;
        }
    }
    false
}
fn main() {
    let input = include_str!("input.txt");
    let tests: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let (goal, nums) = line.split_once(": ").unwrap();
            let goal: u64 = goal.parse().unwrap();
            let nums: Vec<u64> = nums.split(" ").map(|n| n.parse().unwrap()).collect();
            (goal, nums)
        })
        .collect();
    
    let mut res1: u64 = 0;
    for (goal, nums) in tests.clone() {
        if test_valid(goal, nums, vec![Op::Add, Op::Mul]) {
            res1 += goal;
        }
    }
    println!("Part 1: {}", res1);

    let mut res2: u64 = 0;
    for (goal, nums) in tests {
        if test_valid(goal, nums, vec![Op::Add, Op::Mul, Op::Combine]) {
            res2 += goal;
        }
    }
    println!("Part 2: {}", res2);
}
