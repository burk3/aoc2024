use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let mult_re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)").unwrap();
    let mut res1 = 0;
    for (_, [a_str, b_str]) in mult_re.captures_iter(input).map(|c| c.extract()) {
        let a: i32 = a_str.parse().unwrap();
        let b: i32 = b_str.parse().unwrap();
        res1 += a * b;
    }
    println!("{}", res1);

    // part 2
    // regex hax
    let logic_re = Regex::new(r"(mul)\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|(do(?:n't)?)\(()()\)").unwrap();
    let mut multiplying = true;
    let mut res2 = 0;
    for (_, [op, a_str, b_str]) in logic_re.captures_iter(input).map(|c| c.extract()) {
        match op {
            "do" => multiplying = true,
            "don't" => multiplying = false,
            "mul" if multiplying => {
                let a: i32 = a_str.parse().unwrap();
                let b: i32 = b_str.parse().unwrap();
                res2 += a * b;
            },
            _ => (),
        }
    }
    println!("{}", res2);
}
