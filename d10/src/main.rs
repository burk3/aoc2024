use std::{collections::HashSet, fmt::Debug};

use coord_2d::{Coord, Size};

struct TopoMap {
    map: Vec<Vec<u32>>,
    size: Size,
}
impl TopoMap {
    fn new(map: Vec<Vec<u32>>) -> Self {
        let size = Size::new(map[0].len() as u32, map.len() as u32);
        TopoMap { map, size }
    }
}
impl std::ops::Index<Coord> for TopoMap {
    type Output = u32;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.map[index.y as usize][index.x as usize]
    }
}

#[derive(Clone)]
struct PathNode {
    pos: Coord,
    height: u32,
}

#[derive(Clone)]
struct Cursor {
    pos: Coord,
    path: Vec<PathNode>,
    height: u32,
}
impl Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.iter().map(|c| format!("({},{})[{}]", c.pos.x, c.pos.y, c.height)).collect::<Vec<String>>().join("|");
        write!(f, "pos:{:?}, height:{} path:{:}", self.pos, path, self.height)
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Part1Point {
    trailhead: Coord,
    summit: Coord,
}

impl Cursor {
    fn new(start_pos: Coord, height: u32) -> Self {
        Cursor {
            pos: start_pos,
            path: vec![PathNode{pos: start_pos, height}], // Initialize path with starting position
            height
        }
    }
    fn new_with_path(pos: Coord, path: Vec<PathNode>, height: u32) -> Self {
        Cursor {
            pos,
            path,
            height
        }
    }
    fn is_done(&self) -> bool {
        // Check if the cursor is at the end of the path
        self.height == 9
    }
    fn add_move(&self, map: &TopoMap, new_pos: Coord) -> Cursor {
        let mut new_path = self.path.clone();
        let height = map[new_pos];
        new_path.push(PathNode{pos: new_pos, height});
        Cursor::new_with_path(new_pos, new_path, height)
    }
    fn valid_moves(&self, map: &TopoMap) -> Vec<Coord> {
        let mut moves: Vec<Coord> = Vec::new();
        let up = self.pos + Coord::new(0, -1);
        let down = self.pos + Coord::new(0, 1);
        let left = self.pos + Coord::new(-1, 0);
        let right = self.pos + Coord::new(1, 0);
        if up.is_valid(map.size) && map[up] == self.height + 1 {
            moves.push(up);
        }
        if down.is_valid(map.size) && map[down] == self.height + 1 {
            moves.push(down);
        }
        if left.is_valid(map.size) && map[left] == self.height + 1 {
            moves.push(left);
        }
        if right.is_valid(map.size) && map[right] == self.height + 1 {
            moves.push(right);
        }
        moves
    }
    fn step(&self, map: &TopoMap) -> Vec<Cursor> {
        if self.is_done() {
            return vec![self.clone()];
        }
        self.valid_moves(&map).iter().map(|&pos| self.add_move(map, pos)).collect()
    }
}

fn main() {
    let input = include_str!("input.txt");

    // parse input into a 2d vec. track trailheads as we go
    let mut input_map: Vec<Vec<u32>> = Vec::new();
    let mut trailheads: Vec<Coord> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let n = c.to_digit(10).unwrap();
            if n == 0 {
                trailheads.push(Coord::new(x as i32, y as i32));
            }
            row.push(n);
        }
        input_map.push(row);
    }
    let map = TopoMap::new(input_map);

    // part 1
    // loop over trailheads, making a cursor for each
    let init_cursors: Vec<Cursor> = trailheads.iter().map(|&pos| Cursor::new(pos, map[pos])).collect();
    let mut cursors = init_cursors;
    while cursors.iter().any(|c| !c.is_done()) {
        let new_cursors: Vec<Cursor> = cursors.iter().flat_map(|c| c.step(&map)).collect();
        cursors = new_cursors;
    }
    // now cursors contains all paths from all trailheads to all summits. deduplicate on (head, summit) and count to get the score
    let mut p1points: HashSet<Part1Point> = HashSet::new();
    cursors.iter().for_each(|c| {
        let head = c.path[0].pos;
        let summit = c.path.last().unwrap().pos;
        p1points.insert(Part1Point{trailhead: head, summit});
    });
    println!("part 1: {}", p1points.len());
    println!("part 2: {}", cursors.len())
    
}
