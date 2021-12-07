use advent2021::*;
use rayon::prelude::*;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .stack_size(16_000_000)
        .build_global()
        .unwrap();

    let funcs: &'static [fn()] = &[
        day01_a, day01_b, day02_a, day02_b, day03_a, day03_b, day04, day05_a, day05_b, day06,
        day07_a, day07_b, day08_a, day08_b, day09_a, day09_b, day10_a, day10_b, day11_a, day11_b,
        day12_a, day12_b, day13_a, day13_b, day14_a, day14_b, day15_a, day15_b, day16_a, day16_b,
        day17_a, day17_b, day18_a, day18_b, day19_a, day19_b, day20_a, day20_b, day21_a, day21_b,
        day22_a, day22_b, day23_a, day23_b, day24_a, day24_b, day25_a, day25_b,
    ];

    (0..funcs.len()).into_par_iter().for_each(|i| funcs[i]());
}
