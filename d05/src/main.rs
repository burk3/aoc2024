use core::panic;
use std::collections::HashMap;

fn parse_pair(s: &str) -> (i32, i32) {
    let mut split = s.split('|');
    let start = split.next().unwrap().parse().unwrap();
    let end = split.next().unwrap().parse().unwrap();
    (start, end)
}
fn parse_list(s: &str) -> Vec<i32> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn verify_page_set(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    // index page -> position in page_set
    let mut page_index: HashMap<i32, i32> = HashMap::new();
    update.iter().enumerate().for_each(|(i, page)| {
        page_index.insert(*page, i as i32);
    });
    // check rules
    for (page, index) in page_index.iter() {
        if let Some(afters) = rules.get(page) {
            for after in afters.iter() {
                if let Some(after_index) = page_index.get(after) {
                    if after_index < index {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn fix_page_set(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> i32 {
    // index page -> position in page_set
    let mut fixed = update.clone();
    // check rules
    while !verify_page_set(rules, &fixed) {
        let mut page_index: HashMap<i32, i32> = HashMap::new();
        fixed.iter().enumerate().for_each(|(i, page)| {
            page_index.insert(*page, i as i32);
        });
        'outer: for (page, index) in page_index.iter() {
            if let Some(afters) = rules.get(page) {
                for after in afters.iter() {
                    if let Some(after_index) = page_index.get(after) {
                        if after_index < index {
                            fixed.swap(*index as usize, *after_index as usize);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    fixed[fixed.len()/2]
}

fn main() {
    let input = include_str!("input.txt");
    
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // loop over lines
    let mut line_it = input.lines();
    // parse pairs until empty line
    for line in &mut line_it {
        if line.is_empty() {
            break;
        }
        let (before, after) = parse_pair(line);
        if let Some(afters) = rules.get_mut(&before) {
            afters.push(after);
        } else {
            rules.insert(before, vec![after]);
        }
    }
    // resume iteration parsing lists
    for line in line_it {
        updates.push(parse_list(line));
    }

    let mut res1 = 0;
    let mut res2 = 0;
    for update in updates.iter() {
        if verify_page_set(&rules, update) {
            let middle_page = update[update.len()/2];
            res1 += middle_page;
        } else {
            res2 += fix_page_set(&rules, update);
        }
    }
    println!("{}", res1);
    println!("{}", res2);

}
