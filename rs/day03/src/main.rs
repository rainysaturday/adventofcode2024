use std::env::current_dir;

fn main() {
    let instructions = include_str!("../../../inputs/day03").to_string();

    println!(
        "part1 : {}",
        parse_muls(&instructions, true)
            .iter()
            .map(|i| match i {
                Insn::Mul(a, b) => a * b,
            })
            .sum::<usize>()
    );

    println!(
        "part2: {}",
        parse_muls(&instructions, false)
            .iter()
            .map(|i| match i {
                Insn::Mul(a, b) => a * b,
            })
            .sum::<usize>()
    );
}

fn parse_muls(data: &str, always_enabled: bool) -> Vec<Insn> {
    let mut insns = Vec::new();
    let mut enabled = true;
    let r = regex::Regex::new(r#"^mul\((\d+),(\d+)\)"#).unwrap();

    for i in 0..data.len() {
        let current_str = &data[i..];

        if current_str.starts_with("do()") {
            enabled = true;
        } else if current_str.starts_with("don't()") {
            enabled = false;
        } else if enabled || always_enabled {
            for c in r.captures_iter(current_str) {
                let a = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let b = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                insns.push(Insn::Mul(a, b));
            }
        }
    }
    insns
}

enum Insn {
    Mul(usize, usize),
}
