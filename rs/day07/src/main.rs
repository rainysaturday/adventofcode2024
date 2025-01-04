struct Equation {
    ans: i64,
    parts: Vec<i64>,
}

#[derive(Debug, Copy, Clone)]
enum Oper {
    Add,
    Mul,
    Concat,
}

fn main() {
    let eqs = include_str!("../../../inputs/day07.txt")
        .lines()
        .map(|l| {
            let parts = l
                .replace(":", "")
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Equation {
                ans: parts[0],
                parts: parts[1..].to_vec(),
            }
        })
        .collect::<Vec<Equation>>();

    let sum_of_true = eqs
        .iter()
        .map(|e| {
            if let Some(_sol) = get_opers(e, &[Oper::Add, Oper::Mul]) {
                e.ans
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("part1: {sum_of_true}");

    let sum_of_true_w_concat = eqs
        .iter()
        .map(|e| {
            if let Some(_sol) = get_opers(e, &[Oper::Add, Oper::Mul, Oper::Concat]) {
                e.ans
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("part2: {}", sum_of_true_w_concat);
}

fn get_opers(eq: &Equation, allowed_opers: &[Oper]) -> Option<Vec<Oper>> {
    try_oper(eq.ans, eq.parts[0], &eq.parts[1..], allowed_opers)
}

fn try_oper(ans: i64, acc: i64, rest: &[i64], allowed_opers: &[Oper]) -> Option<Vec<Oper>> {
    if rest.is_empty() {
        if ans == acc {
            Some(Vec::new())
        } else {
            None
        }
    } else {
        let popped = rest[0];
        for oper in allowed_opers {
            let new_acc = match oper {
                Oper::Add => acc + popped,
                Oper::Mul => acc * popped,
                Oper::Concat => concat(acc, popped),
            };

            if let Some(mut successopers) = try_oper(ans, new_acc, &rest[1..], allowed_opers) {
                successopers.insert(0, *oper);
                return Some(successopers);
            }
        }
        None
    }
}

fn concat(a: i64, b: i64) -> i64 {
    let digits = b.ilog10() + 1;
    (a * (10i64.pow(digits))) + b
}
