fn main() {
    let levels = include_str!("../../../inputs/day02.txt")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!(
        "Part1: safe reports: {}",
        levels.iter().filter(|level| is_safe(level)).count()
    );

    println!(
        "Part2: safe reports: {}",
        levels
            .iter()
            .filter(|level| is_safe_dampened(level))
            .count()
    );
}

fn is_safe_dampened(level: &[i32]) -> bool {
    if is_safe(level) {
        return true;
    }

    // Very brute-force
    for i in 0..level.len() {
        let mut dampened_level = level.to_vec();
        dampened_level.remove(i);
        if is_safe(&dampened_level) {
            return true;
        }
    }
    false
}

fn is_safe(level: &[i32]) -> bool {
    let init_ascending = level[1] > level[0];
    for pos in 1..level.len() {
        let current = level[pos];
        let prev = level[pos - 1];

        let diff = current.abs_diff(prev);
        if diff > 3 || diff == 0 {
            return false;
        }
        let curr_ascending = current > prev;
        if curr_ascending != init_ascending {
            return false;
        }
    }
    true
}
