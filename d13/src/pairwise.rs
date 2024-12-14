use num_traits::Zero;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Rem, Sub},
};

const A_COINS: i64 = 3;
const B_COINS: i64 = 1;
const HARD_DIFF: i64 = 10000000000000;

#[derive(PartialEq, Copy, Clone)]
pub struct Pair {
    x: i64,
    y: i64,
}

impl Pair {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn any_negative(&self) -> bool {
        self.x < i64::zero() || self.y < i64::zero()
    }
    fn same(&self) -> bool {
        self.x == self.y
    }
}
impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Div<i64> for Pair {
    type Output = Pair;
    fn div(self, rhs: i64) -> Pair {
        Pair::new(self.x / rhs, self.y / rhs)
    }
}
impl Div<Pair> for Pair {
    type Output = Pair;
    fn div(self, rhs: Pair) -> Pair {
        Pair::new(self.x / rhs.x, self.y / rhs.y)
    }
}
impl Mul<i64> for Pair {
    type Output = Pair;
    fn mul(self, rhs: i64) -> Pair {
        Pair::new(self.x * rhs, self.y * rhs)
    }
}
impl Add<Pair> for Pair {
    type Output = Pair;
    fn add(self, other: Pair) -> Pair {
        Pair::new(self.x + other.x, self.y + other.y)
    }
}
impl Add<i64> for Pair {
    type Output = Pair;
    fn add(self, other: i64) -> Pair {
        Pair::new(self.x + other, self.y + other)
    }
}
impl Sub<Pair> for Pair {
    type Output = Pair;
    fn sub(self, other: Pair) -> Pair {
        Pair::new(self.x - other.x, self.y - other.y)
    }
}
impl Rem<Pair> for Pair {
    type Output = Pair;
    fn rem(self, other: Pair) -> Pair {
        Pair::new(self.x % other.x, self.y % other.y)
    }
}
impl Zero for Pair {
    fn zero() -> Self {
        Pair::new(i64::zero(), i64::zero())
    }
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

pub type Prize = Pair;
pub type Button = Pair;

#[derive(Debug)]
pub struct Machine {
    a: Pair,
    b: Button,
    prize: Prize,
}
impl Machine {
    pub fn new(a: Button, b: Button, prize: Prize) -> Self {
        Self { a, b, prize }
    }
    pub fn solvable(&self) -> bool {
        for num_a in 0..101 {
            let check = self.prize - self.a * num_a;
            if check.any_negative() {
                break;
            }
            if check % self.b == Prize::zero() && (check / self.b).same() {
                return true;
            }
        }
        false
    }
    pub fn cheapest(&self) -> (i64, i64) {
        let mut cheapest = (0, 0);
        let mut cheapest_cost = i64::MAX;
        for num_a in 0..101 {
            let check = self.prize - self.a * num_a;
            if check.any_negative() {
                break;
            }
            if check % self.b != Pair::zero() {
                continue;
            }
            let check = check / self.b;
            if !check.same() {
                continue;
            }
            let num_b = check.x;
            let cost = num_a * A_COINS + num_b * B_COINS;
            if cost < cheapest_cost {
                cheapest = (num_a, num_b);
                cheapest_cost = cost;
            }
        }
        cheapest
    }
    pub fn harder_cheapest(&self) -> Option<(i64, i64)> {
        // this sucks
        let prize = self.prize + HARD_DIFF;
        let numer_b = prize.x * self.a.y - prize.y * self.a.x;
        let denom_b = self.b.x * self.a.y - self.b.y * self.a.x;
        if denom_b == 0 || numer_b % denom_b != 0 {
            return None;
        }
        let b_presses = numer_b / denom_b;
        let numer_a = prize.x - self.b.x * b_presses;
        let denom_a = self.a.x;
        if denom_a == 0 || numer_a % denom_a != 0 {
            return None;
        }
        let a_presses = numer_a / denom_a;

        Some((a_presses, b_presses))
    }
}
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (gcd, x, y)
}
