fn main() {
    let lines = include_str!("../../../inputs/day04.txt")
        .lines()
        .collect::<Vec<&str>>();

    println!("part1: {}", get_xmas_pos(&lines).len());
    println!("part2: {}", get_cross_mass_pos(&lines).len());
}

fn get_xmas_pos(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for y in 0..lines.len() {
        let current_line = lines[y];
        for x in 0..current_line.len() {
            let all_dirs = [
                dline(lines, x, y, 1, 0, 4),  // hori line
                dline(lines, x, y, 0, 1, 4),  // vert line
                dline(lines, x, y, 1, 1, 4),  // diag line down
                dline(lines, x, y, 1, -1, 4), // diag line up
            ];

            for _ in all_dirs.iter().filter(|l| contains_xmas(l)) {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn get_cross_mass_pos(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for y in 0..lines.len() {
        let current_line = lines[y];
        for x in 0..current_line.len() {
            let all_dirs = [
                dline(lines, x, y, 1, 1, 3),      // diag line down into bottom right
                dline(lines, x, y + 2, 1, -1, 3), // diag line up into top right
            ];

            if all_dirs.iter().all(|l| contains_mas(l)) {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn dline(lines: &[&str], x: usize, y: usize, dx: usize, dy: i32, len: usize) -> String {
    let mut res = String::new();
    let height = lines.len();
    let width = lines[0].len();
    let y: i32 = y as i32;
    for offset in 0..len {
        let x = x + (dx * offset);
        let y: i32 = y + (dy * offset as i32);

        if y < 0 {
            continue;
        }

        let y = y as usize;
        if x < width && y < height {
            res.push(lines.get(y).unwrap().chars().nth(x).unwrap());
        }
    }
    res
}

fn contains_mas(line: &str) -> bool {
    let x = "MAS";
    let rx = "SAM";
    x == line || rx == line
}

fn contains_xmas(line: &str) -> bool {
    let x = "XMAS";
    let rx = "SAMX";
    x == line || rx == line
}
