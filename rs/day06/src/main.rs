use std::collections::HashSet;

fn main() {
    let lines = include_str!("../../../inputs/day06.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut p1world = lines.clone();
    let dir = (0, -1); // up
    let pos = guard_pos(&p1world).unwrap();
    let walked = walk_until_loop_or_outside(&p1world, pos, dir, false).0;
    render(&p1world);
    println!("part1: visited {}", walked.len());

    // for part 2, try inserting an obstable on every walked position (except starting)
    let mut loop_locations = 0;
    print!("Checking pos");
    for (index, walked_pos) in walked.iter().enumerate() {
        print!("\rChecking pos {index}, {loop_locations}");
        if p1world[walked_pos.1 as usize][walked_pos.0 as usize] != '.' {
            continue; // only insert obstacles on empty slots
        }
        p1world[walked_pos.1 as usize][walked_pos.0 as usize] = 'O';

        if walk_until_loop_or_outside(&p1world, pos, dir, true).1 {
            loop_locations += 1;
        }

        // reset to empty after
        p1world[walked_pos.1 as usize][walked_pos.0 as usize] = '.';
    }
    println!("\npart2: looping locations {loop_locations}");
}

fn walk_until_loop_or_outside(
    p1world: &[Vec<char>],
    pos: (i32, i32),
    dir: (i32, i32),
    check_loop: bool,
) -> (HashSet<(i32, i32)>, bool) {
    // create local mutable versions
    let mut pos = pos;
    let mut dir = dir;
    let mut walked: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    loop {
        if check_loop && walked.contains(&(pos, dir)) {
            // we have been exactly here before, also an exit condition
            return (walked.iter().map(|v| v.0).collect(), true);
        }
        walked.insert((pos, dir));
        walk_guard(p1world, &mut pos, &mut dir);

        if is_outside(p1world, &pos) {
            return (walked.iter().map(|v| v.0).collect(), false);
        }
    }
}

fn render(world: &[Vec<char>]) {
    let rendered = world
        .iter()
        .map(|l| l.iter().map(|c| c.to_string()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{rendered}");
}

fn is_outside(world: &[Vec<char>], pos: &(i32, i32)) -> bool {
    if pos.0 < 0 || pos.0 >= world[0].len() as i32 {
        return true;
    }

    if pos.1 < 0 || pos.1 >= world.len() as i32 {
        return true;
    }

    false
}

///
/// Walks one step and paints behind
///
fn walk_guard(world: &[Vec<char>], pos: &mut (i32, i32), dir: &mut (i32, i32)) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if is_outside(world, &new_pos) {
        pos.0 = new_pos.0;
        pos.1 = new_pos.1;
    } else if world[new_pos.1 as usize][new_pos.0 as usize] == '#'
        || world[new_pos.1 as usize][new_pos.0 as usize] == 'O'
    {
        // just rotate without walking, next iteration will walk the guard
        rotate(dir);
    } else {
        // just update to the new position
        pos.0 = new_pos.0;
        pos.1 = new_pos.1;
    }
}

fn rotate(dir: &mut (i32, i32)) {
    match dir {
        (0, -1) => {
            dir.0 = 1;
            dir.1 = 0;
        }
        (1, 0) => {
            dir.0 = 0;
            dir.1 = 1;
        }
        (0, 1) => {
            dir.0 = -1;
            dir.1 = 0;
        }
        (-1, 0) => {
            dir.0 = 0;
            dir.1 = -1;
        }
        d => panic!("invalid dir {d:?}"),
    }
}

fn guard_pos(world: &[Vec<char>]) -> Option<(i32, i32)> {
    for (y, l) in world.iter().enumerate() {
        if let Some(i) = l.iter().enumerate().find(|(_, c)| **c == '^') {
            return Some((i.0 as i32, y as i32));
        }
    }
    None
}
