mod pairwise;

use std::result;

use pairwise::{Button, Machine, Prize};
use regex::Regex;

const A_COINS: i64 = 3;
const B_COINS: i64 = 1;

fn parse_machines(input: &str) -> Vec<Machine> {
    let button_re: Regex = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut line_iter = input.lines();
    let mut machines = Vec::new();
    while let Some(button_a) = button_re.captures(line_iter.next().unwrap()) {
        let a = Button::new(button_a[2].parse().unwrap(), button_a[3].parse().unwrap());
        let button_b = button_re.captures(line_iter.next().unwrap()).unwrap();
        let b = Button::new(button_b[2].parse().unwrap(), button_b[3].parse().unwrap());
        let prize = prize_re.captures(line_iter.next().unwrap()).unwrap();
        let prize = Prize::new(prize[1].parse().unwrap(), prize[2].parse().unwrap());
        machines.push(Machine::new(a, b, prize));
        if let None = line_iter.next() {
            break;
        }
    }
    machines
}
fn main() {
    let input = include_str!("input.txt");
    let machines = parse_machines(input);

    let mut result: i64 = 0;
    for machine in machines.iter() {
        if machine.solvable() {
            let (num_a, num_b) = machine.cheapest();
            result += num_a * A_COINS + num_b * B_COINS;
        }
    }
    println!("{}", result);

    let mut result: i64 = 0;
    for machine in machines.iter() {
        if let Some((a_presses, b_presses)) = machine.harder_cheapest() {
            result += a_presses * A_COINS + b_presses * B_COINS;
        }
    }
    println!("{}", result);
}
