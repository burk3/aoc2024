use direction::CardinalDirection as Dir;

#[derive(Clone, PartialEq)]
enum Space {
    Empty(Option<Dir>),
    Wall,
}
enum State {
    End,
    Walking,
    Looped,
}

#[derive(Clone)]
struct MapRunner {
    map: Vec<Vec<Space>>,
    x: usize,
    y: usize,
    direction: Dir,
    loop_found: bool,
}

impl MapRunner {
    fn from_input(input: &str) -> Self {
        let mut map = Vec::new();
        let mut new_x : usize = 0;
        let mut new_y : usize = 0;
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => row.push(Space::Wall),
                    '.' => row.push(Space::Empty(None)),
                    '^' => {new_x = x; new_y = y; row.push(Space::Empty(Some(Dir::North)));},
                    _ => panic!("Invalid character in input"),
                }
            }
            map.push(row);
        }
        Self{
            map,
            x: new_x,
            y: new_y,
            direction: Dir::North,
            loop_found: false,
        }
    }

    // Step the runner one step forward.
    // if we land on a space that we've already been to and going the same direction, we've looped.
    fn step(&mut self) -> State {
        match self.direction {
            Dir::North if self.y == 0 => return State::End,
            Dir::East if self.x == self.map[0].len() - 1 => return State::End,
            Dir::South if self.y == self.map.len() - 1 => return State::End,
            Dir::West if self.x == 0 => return State::End,
            _ => (),
        }
        let in_front = match self.direction {
            Dir::North => &self.map[self.y - 1][self.x],
            Dir::East => &self.map[self.y][self.x + 1],
            Dir::South => &self.map[self.y + 1][self.x],
            Dir::West => &self.map[self.y][self.x - 1],
        };
        match in_front {
            Space::Wall => {
                self.direction = self.direction.right90();
            },
            Space::Empty(Some(d)) if *d == self.direction => return State::Looped,
            Space::Empty(_) => {
                match self.direction {
                    Dir::North => self.y -= 1,
                    Dir::East => self.x += 1,
                    Dir::South => self.y += 1,
                    Dir::West => self.x -= 1,
                }
                self.map[self.y][self.x] = Space::Empty(Some(self.direction));
            },
        }
        State::Walking
    }

    fn run(&mut self) {
        loop {
            match self.step() {
                State::End => break,
                State::Walking => (),
                State::Looped => {
                    if self.loop_found {
                        break;
                    }
                    self.loop_found = true;
                },
            }
        }
    }

    fn count_visited(&self) -> usize {
        self.map.iter().flatten().filter(|s| {
            match s {
                Space::Empty(Some(_)) => true,
                _ => false,
            }
        }).count()
    }
}
fn main() {
    let input = include_str!("input.txt");
    let mut runner = MapRunner::from_input(input);
    // for part 2
    let runner2 = runner.clone();

    runner.run();
    println!("Visited: {}", runner.count_visited());

    let mut res2 = 0;
    for y in 0..runner2.map.len() {
        for x in 0..runner2.map[0].len() {
            if runner2.map[y][x] != Space::Empty(None) {
                continue;
            }
            let mut runner = runner2.clone();
            runner.map[y][x] = Space::Wall;
            runner.run();
            if runner.loop_found {res2 += 1;}
        }
    }
    println!("Part 2: {}", res2);
}
