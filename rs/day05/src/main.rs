use core::panic;

fn main() {
    let lines = include_str!("../../../inputs/day05.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut values: Vec<Vec<usize>> = Vec::new();

    // parse
    let mut passed_empty = false;
    for line in lines {
        if line.is_empty() {
            passed_empty = true;
            continue;
        }

        if passed_empty {
            // values
            values.push(
                line.split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect(),
            );
        } else {
            // rules
            let r = line
                .split('|')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            rules.push((r[0], r[1]));
        }
    }

    let sum = values
        .iter()
        .filter(|values| is_valid(values, &rules))
        .map(|values| {
            let mid = values[values.len() / 2];
            if values.len() % 2 == 0 {
                panic!("not odd length");
            }
            mid
        })
        .sum::<usize>();
    println!("part1: {}", sum);

    let sum = values
        .iter()
        .filter(|values| !is_valid(values, &rules))
        .map(|values| correct(values, &rules))
        .map(|values| {
            let mid = values[values.len() / 2];
            if values.len() % 2 == 0 {
                panic!("not odd length");
            }
            mid
        })
        .sum::<usize>();
    println!("part2: {}", sum);
}

fn correct(values: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    // find set of rules relevant for this set of values
    let relevant = rules
        .iter()
        .cloned()
        .filter(|rule| values.contains(&rule.0) && values.contains(&rule.1))
        .collect::<Vec<(usize, usize)>>();

    let mut corrected = values.to_vec();

    for _ in 0..1000 {
        if is_valid(&corrected, &relevant) {
            return corrected;
        }

        // go through all rules until we need to put something in front
        // kind of like a reverse bubble sort
        for rule in &relevant {
            let search1 = corrected.iter().enumerate().find_map(|(index, v)| {
                if *v == rule.0 {
                    Some(index)
                } else {
                    None
                }
            });
            let search2 = corrected.iter().enumerate().find_map(|(index, v)| {
                if *v == rule.1 {
                    Some(index)
                } else {
                    None
                }
            });

            if let (Some(a), Some(b)) = (search1, search2) {
                if a > b {
                    // then insert b in front of a
                    let b_val = corrected.remove(b);
                    corrected.insert(a, b_val);
                    break; // let-s run throug the rules again
                }
            }
        }
    }

    panic!("failed to correct in 1000")
}

fn is_valid(values: &[usize], rules: &[(usize, usize)]) -> bool {
    rules.iter().all(|rule| is_rule_valid(values, rule))
}

fn is_rule_valid(values: &[usize], rule: &(usize, usize)) -> bool {
    let search1 = values
        .iter()
        .enumerate()
        .filter_map(|(index, v)| if *v == rule.0 { Some(index) } else { None })
        .collect::<Vec<usize>>();
    let search2 = values
        .iter()
        .enumerate()
        .filter_map(|(index, v)| if *v == rule.1 { Some(index) } else { None })
        .collect::<Vec<usize>>();

    if search1.len() > 2 || search2.len() > 2 {
        panic!("scary");
    }

    match (search1.first(), search2.first()) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}
