fn check_sequence(levels: &[i32]) -> bool {
    let differences: Vec<i32> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();

    differences.windows(2).all(|pair| {
        let diff1 = pair[0].abs();
        let diff2 = pair[1].abs();
        if diff1 > 3 || diff2 > 3 || diff1 < 1 || diff2 < 1 {
            false
        } else {
            pair[0].signum() == pair[1].signum()
        }
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input.lines().filter(|line| {
        let levels: Vec<i32> = line.split_whitespace().map(|item| item.parse().unwrap()).collect();
        check_sequence(&levels)
    }).count() as u32
}

fn is_safe_after_removal(levels: &[i32]) -> bool {
    // Try removing each element one by one and check if the sequence becomes valid
    for index in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(index);
        if check_sequence(&modified_levels) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input.lines().filter(|line| {
        let levels: Vec<i32> = line.split_whitespace().map(|item| item.parse().unwrap()).collect();
        check_sequence(&levels) || is_safe_after_removal(&levels)
    }).count() as u32
}


