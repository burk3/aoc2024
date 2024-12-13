use std::fmt::{self, Debug, Formatter};
use std::{collections::BTreeSet, collections::HashSet, ops::Index};

use coord_2d::{Coord, Size};

const UP: Coord = Coord::new(0, -1);
const DOWN: Coord = Coord::new(0, 1);
const LEFT: Coord = Coord::new(-1, 0);
const RIGHT: Coord = Coord::new(1, 0);

struct AgMap {
    map: Vec<Vec<char>>,
    size: Size,
}
impl AgMap {
    fn new(map: Vec<Vec<char>>) -> Self {
        let size = Size::new(map[0].len() as u32, map.len() as u32);
        Self { map, size }
    }
}

impl Index<Coord> for AgMap {
    type Output = char;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.map[index.y as usize][index.x as usize]
    }
}

struct Region {
    crop: char,
    coords: HashSet<Coord>,
    edges: HashSet<Coord>,
}
impl Region {
    fn new(crop: char) -> Self {
        Self {
            crop,
            coords: HashSet::new(),
            edges: HashSet::new(),
        }
    }
    fn expand(&mut self, proposed: Coord, map: &AgMap, visited: &mut HashSet<Coord>) {
        self.expand_(proposed, map, visited);
        // filter out wrong edges
        self.edges = self
            .edges
            .clone()
            .into_iter()
            .filter(|&pos| !self.coords.contains(&pos) && self.is_edge(pos))
            .collect();
    }
    fn expand_(&mut self, proposed: Coord, map: &AgMap, visited: &mut HashSet<Coord>) {
        if proposed.is_valid(map.size) && !visited.contains(&proposed) && map[proposed] == self.crop
        {
            self.coords.insert(proposed);
            visited.insert(proposed);
            self.expand_(proposed + UP, map, visited);
            self.expand_(proposed + DOWN, map, visited);
            self.expand_(proposed + LEFT, map, visited);
            self.expand_(proposed + RIGHT, map, visited);
        } else {
            self.edges.insert(proposed);
        }
    }
    fn is_edge(&self, pos: Coord) -> bool {
        self.coords.contains(&(pos + UP))
            || self.coords.contains(&(pos + DOWN))
            || self.coords.contains(&(pos + LEFT))
            || self.coords.contains(&(pos + RIGHT))
    }
    fn area(&self) -> usize {
        self.coords.len()
    }
    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for pos in self.edges.iter() {
            if self.coords.contains(&(pos + UP)) {
                perimeter += 1;
            }
            if self.coords.contains(&(pos + DOWN)) {
                perimeter += 1;
            }
            if self.coords.contains(&(pos + LEFT)) {
                perimeter += 1;
            }
            if self.coords.contains(&(pos + RIGHT)) {
                perimeter += 1;
            }
        }
        perimeter
    }
    fn sides(&self) -> usize {
        let mut lefts: BTreeSet<Coord> = BTreeSet::new();
        let mut rights: BTreeSet<Coord> = BTreeSet::new();
        let mut ups: BTreeSet<Coord> = BTreeSet::new();
        let mut downs: BTreeSet<Coord> = BTreeSet::new();
        for edge in self.edges.iter() {
            if self.coords.contains(&(edge + LEFT)) {
                rights.insert(*edge);
            }
            if self.coords.contains(&(edge + RIGHT)) {
                lefts.insert(*edge);
            }
            if self.coords.contains(&(edge + UP)) {
                downs.insert(*edge);
            }
            if self.coords.contains(&(edge + DOWN)) {
                ups.insert(*edge);
            }
        }
        let mut sides = 0;
        for edge in lefts.iter() {
            if !lefts.contains(&(edge + UP)) {
                sides += 1;
            }
        }
        for edge in rights.iter() {
            if !rights.contains(&(edge + UP)) {
                sides += 1;
            }
        }
        for edge in ups.iter() {
            if !ups.contains(&(edge + LEFT)) {
                sides += 1;
            }
        }
        for edge in downs.iter() {
            if !downs.contains(&(edge + LEFT)) {
                sides += 1;
            }
        }
        sides
    }
}
impl Debug for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Region of {} plants with price {} * {} = {}",
            self.crop,
            self.coords.len(),
            self.sides(),
            self.area() * self.perimeter()
        )
    }
}

fn gen_regions(map: &AgMap) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    for pos in map.size.coord_iter_row_major() {
        if visited.contains(&pos) {
            continue;
        }
        let mut new_region = Region::new(map[pos]);
        new_region.expand(pos, map, &mut visited);
        regions.push(new_region);
    }
    regions
}

fn main() {
    let input = include_str!("ex_input.txt");
    let map = AgMap::new(input.lines().map(|line| line.chars().collect()).collect());
    let regions = gen_regions(&map);

    let result1: usize = regions
        .iter()
        .map(|region| region.area() * region.perimeter())
        .sum();
    println!("Part 1: {}", result1);
    // regions.iter().for_each(|region| println!("{:?}", region));
    let result2: usize = regions
        .iter()
        .map(|region| region.area() * region.sides())
        .sum();
    println!("Part 2: {}", result2);
}
