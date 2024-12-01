fn main() {
    let lines = include_str!("../../../inputs/day01.txt")
        .lines()
        .collect::<Vec<&str>>();

    let pairs = lines
        .iter()
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect::<Vec<(u32, u32)>>();

    let mut l1 = pairs.iter().map(|v| v.0).collect::<Vec<u32>>();
    let mut l2 = pairs.iter().map(|v| v.1).collect::<Vec<u32>>();

    l1.sort();
    l2.sort();

    let dist = l1
        .iter()
        .zip(l2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();
    println!("Part1: diff {dist}");

    let sim_score = l1
        .iter()
        .map(|l| l * l2.iter().filter(|r| *r == l).count() as u32)
        .sum::<u32>();
    println!("Part2: sim_score: {sim_score}");
}
