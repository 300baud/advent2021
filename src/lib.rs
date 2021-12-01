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

    println!("Part 01a saw {} measurements that were deeper", count);
}

pub fn day01_b() {
    let count = include_str!("input/day01")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .as_slice()
        .windows(4)
        .filter(|r| r[1..=3].iter().sum::<u32>() > r[0..=2].iter().sum::<u32>())
        .count();

    println!("Part 01a saw {} measurements that were deeper", count);
}
