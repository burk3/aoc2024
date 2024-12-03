

// communicate direction for checking report
#[derive(PartialEq, Debug)]
enum Dir {
    Flat,
    Up,
    Down,
}

// calculate direction and distance
fn diff(a: i32, b: i32) -> (Dir, i32) {
    if a > b {
        (Dir::Down, a - b)
    } else if a < b {
        (Dir::Up, b - a)
    } else {
        (Dir::Flat, 0)
    }
}

fn safe_report(report: &Vec<i32>) -> bool {
    // I don't think these exist in the input, but whatever
    if report.len() < 2 {
        return true;
    }
    // start off with the overall direction as flat to not piss off the false checks
    let mut dir = Dir::Flat;
    for i in 1..report.len() {
        match diff(report[i - 1], report[i]) {
            (Dir::Flat, _) => return false,
            // guards are good
            (Dir::Up, n) if dir == Dir::Down || n > 3 => return false,
            (Dir::Down, n) if dir == Dir::Up || n > 3 => return false,
            (Dir::Up, _) => dir = Dir::Up,
            (Dir::Down, _) => dir = Dir::Down,
        }
    }
    true
}

// clone the report and remove the specified element. then check it again``
fn check_with_removal(report: &Vec<i32>, i: usize) -> bool {
    let mut with_removal = report.clone();
    with_removal.remove(i);
    safe_report(&with_removal)
}

// testing by removing either of the failing elements wasnt working, so just
// check all possible removals /shrug
fn check_all_removals(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        if check_with_removal(report, i) {
            return true;
        }
    }
    false
}

fn main() {
    // parse input. ez
    let input = include_str!("input.txt");
    let reports: Vec<Vec<i32>> = input.lines().map(|line| {
        line.split_whitespace().map(|x| x.parse().unwrap()).collect()
    }).collect();

    // part 1
    let mut count = 0;
    for report in &reports {
        if safe_report(&report) {
            count += 1;
        }
    }
    println!("{}", count);

    // part 2
    count = 0;
    for report in reports {
        if safe_report(&report) || check_all_removals(&report) {
            count += 1;
        }
    }
    println!("{}", count);
}
