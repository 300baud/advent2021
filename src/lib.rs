use itertools::Itertools;

pub fn day01_a() {
    let readings = include_str!("input/day01")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut prev: Option<u32> = None;
    let mut count = 0u32;

    for reading in readings {
        if prev.is_some() && reading > prev.unwrap() {
            count += 1;
        }
        prev = Some(reading)
    }

    println!("Part 01a saw {} measurements that were deeper", count);
}

pub fn day01_b() {
    let readings = include_str!("input/day01")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut prev: Option<u32> = None;
    let mut count = 0u32;

    for triple in readings.windows(3) {
        let reading = triple.into_iter().sum();
        if prev.is_some() && reading > prev.unwrap() {
            count += 1;
        }
        prev = Some(reading)
    }

    println!("Part 01a saw {} measurements that were deeper", count);
}
