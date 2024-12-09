use std::collections::{HashSet, HashMap};

use coord_2d::{Coord, Size};
use itertools::Itertools;
use num_rational::Ratio;

fn reduce_int_vec(vec: Coord) -> Coord {
    let rat = Ratio::new(vec.x, vec.y).reduced();
    Coord::new(*rat.numer(), *rat.denom())

}

fn main() {
    let input = include_str!("input.txt");
    let mut antennae: HashMap<char, Vec<Coord>> = HashMap::new();
    let mut w: u32 = 0;
    let mut h: u32= 0;
    for (y, line) in input.lines().enumerate() {
        h = y as u32;
        for (x, c) in line.chars().enumerate() {
            w = x as u32;
            if c != '.' {
                antennae.entry(c).or_insert_with(Vec::new).push(Coord::new(x as i32, y as i32));
            }
        }
    }
    let size = Size::new(w + 1, h + 1);

    // find x and y traven between all pairs of antennae and extend them out in either direction.
    // filter out any that extend beyond the bounds of the grid.
    let mut antinodes: HashSet<Coord> = HashSet::new();
    let mut antinodes2: HashSet<Coord> = HashSet::new();
    for ants in antennae.values() {
        for (a, b) in ants.iter().combinations(2).map(|pair| (pair[0], pair[1])) {
            let diff = a - b;
            let a_prime = a + diff;
            let b_prime = b - diff;
            if size.is_valid(a_prime) {
                antinodes.insert(a_prime);
            }
            if size.is_valid(b_prime) {
                antinodes.insert(b_prime);
            }

            // part 2
            // reduce the vector so we can find "exactly in line" points on the grid
            let diff = reduce_int_vec(diff);
            let mut a_prime = a + diff;
            let mut b_prime = b - diff;
            // then iterate over and over and over
            while size.is_valid(a_prime) {
                antinodes2.insert(a_prime);
                a_prime += diff;
            }
            while size.is_valid(b_prime) {
                antinodes2.insert(b_prime);
                b_prime -= diff;
            }
        }
    }
    println!("antinodes: {}", antinodes.len());
    println!("antinodes2: {}", antinodes2.len());

}
