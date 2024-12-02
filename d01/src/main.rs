use std::collections::HashMap;


fn main() {
    // get the input into the program as a string
    let input = include_str!("input.txt");
    // parse input into two vectors of integers
    let (mut places_a, mut places_b): (Vec<i32>, Vec<i32>) = input.lines().map(|line| {
        let (str_a, str_b) = line.split_once("   ").unwrap();
        let place_a = str_a.parse::<i32>().unwrap();
        let place_b = str_b.parse::<i32>().unwrap();
        (place_a, place_b)
    }).unzip();
    // sort the vectors
    places_a.sort();
    places_b.sort();
    // sum the absolute differences
    let res: i32 = places_a.iter()
        .zip(places_b.iter())
        .map(|(a, b)| (a - b).abs()).sum();
    println!("{}", res);

    // map place ID to count of occurrences for the right list
    let mut b_counts: HashMap<i32, i32> = HashMap::new();
    places_b.iter().for_each(|place| {
        *b_counts.entry(*place).or_insert(0) += 1;
    });
    // sum the products of the place IDs and their counts
    let res2: i32 = places_a.iter()
        .map(|place| place * b_counts.get(place).unwrap_or(&0))
        .sum();
    print!("{}", res2);
}
