use itertools::Itertools;

pub fn day01_a() {
    let count = include_str!("input/day01")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>()
        .as_slice()
        .windows(2)
        .filter(|r| r[1] > r[0])
        .count();

    println!("Problem 01a saw {} measurements that were deeper", count);
}

pub fn day01_b() {
    let count = include_str!("input/day01")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>()
        .as_slice()
        .windows(4)
        .filter(|r| r[1..=3].iter().sum::<u32>() > r[0..=2].iter().sum::<u32>())
        .count();

    println!("Problem 01b saw {} measurements that were deeper", count);
}

pub fn day02_a() {
    let lines = include_str!("input/day02").lines();
    let mut horizontal = 0i32;
    let mut depth = 0i32;

    for line in lines {
        let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let direction = tokens[0];
        let amount: i32 = tokens[1].parse().unwrap();

        match direction {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => {}
        }
    }

    println!(
        "Problem 02a saw a total forward x depth product of {}",
        horizontal * depth
    );
}

pub fn day02_b() {
    println!("day02_b not solved yet!");
}

pub fn day03_a() {
    println!("day03_a not solved yet!");
}

pub fn day03_b() {
    println!("day03_b not solved yet!");
}

pub fn day04_a() {
    println!("day04_a not solved yet!");
}

pub fn day04_b() {
    println!("day04_b not solved yet!");
}

pub fn day05_a() {
    println!("day05_a not solved yet!");
}

pub fn day05_b() {
    println!("day05_b not solved yet!");
}

pub fn day06_a() {
    println!("day06_a not solved yet!");
}

pub fn day06_b() {
    println!("day06_b not solved yet!");
}

pub fn day07_a() {
    println!("day07_a not solved yet!");
}

pub fn day07_b() {
    println!("day07_b not solved yet!");
}

pub fn day08_a() {
    println!("day08_a not solved yet!");
}

pub fn day08_b() {
    println!("day08_b not solved yet!");
}

pub fn day09_a() {
    println!("day09_a not solved yet!");
}

pub fn day09_b() {
    println!("day09_b not solved yet!");
}

pub fn day10_a() {
    println!("day10_a not solved yet!");
}

pub fn day10_b() {
    println!("day10_b not solved yet!");
}

pub fn day11_a() {
    println!("day11_a not solved yet!");
}

pub fn day11_b() {
    println!("day11_b not solved yet!");
}

pub fn day12_a() {
    println!("day12_a not solved yet!");
}

pub fn day12_b() {
    println!("day12_b not solved yet!");
}

pub fn day13_a() {
    println!("day13_a not solved yet!");
}

pub fn day13_b() {
    println!("day13_b not solved yet!");
}

pub fn day14_a() {
    println!("day14_a not solved yet!");
}

pub fn day14_b() {
    println!("day14_b not solved yet!");
}

pub fn day15_a() {
    println!("day15_a not solved yet!");
}

pub fn day15_b() {
    println!("day15_b not solved yet!");
}

pub fn day16_a() {
    println!("day16_a not solved yet!");
}

pub fn day16_b() {
    println!("day16_b not solved yet!");
}

pub fn day17_a() {
    println!("day17_a not solved yet!");
}

pub fn day17_b() {
    println!("day17_b not solved yet!");
}

pub fn day18_a() {
    println!("day18_a not solved yet!");
}

pub fn day18_b() {
    println!("day18_b not solved yet!");
}

pub fn day19_a() {
    println!("day19_a not solved yet!");
}

pub fn day19_b() {
    println!("day19_b not solved yet!");
}

pub fn day20_a() {
    println!("day20_a not solved yet!");
}

pub fn day20_b() {
    println!("day20_b not solved yet!");
}

pub fn day21_a() {
    println!("day21_a not solved yet!");
}

pub fn day21_b() {
    println!("day21_b not solved yet!");
}

pub fn day22_a() {
    println!("day22_a not solved yet!");
}

pub fn day22_b() {
    println!("day22_b not solved yet!");
}

pub fn day23_a() {
    println!("day23_a not solved yet!");
}

pub fn day23_b() {
    println!("day23_b not solved yet!");
}

pub fn day24_a() {
    println!("day24_a not solved yet!");
}

pub fn day24_b() {
    println!("day24_b not solved yet!");
}

pub fn day25_a() {
    println!("day25_a not solved yet!");
}

pub fn day25_b() {
    println!("day25_b not solved yet!");
}