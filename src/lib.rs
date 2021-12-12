use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
    let lines = include_str!("input/day02").lines();
    let mut horizontal = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for line in lines {
        let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let direction = tokens[0];
        let amount: i32 = tokens[1].parse().unwrap();

        match direction {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => {}
        }
    }

    println!(
        "Problem 02b saw a total forward x depth product of {}",
        horizontal * depth
    );
}

struct Day03 {
    line_chars: Box<Vec<Vec<char>>>,
    one_counts: Box<Vec<usize>>,
}

fn day03_helper() -> Box<Day03> {
    let lines = include_str!("input/day03").lines().collect::<Vec<&str>>();
    let num_bits = lines[0].len();

    let line_chars = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let one_counts = (0..num_bits)
        .map(|i| line_chars.iter().filter(|l| l[i] == '1').count())
        .collect_vec();

    Box::new(Day03 {
        line_chars: Box::new(line_chars.clone()),
        one_counts: Box::new(one_counts.clone()),
    })
}

pub fn day03_a() {
    let help = day03_helper();
    let num_bits = help.line_chars.len();
    let one_counts = help.one_counts;
    let mut gamma_bits: Vec<char> = vec![];
    let mut epsilon_bits: Vec<char> = vec![];

    for one_count in one_counts.iter() {
        if *one_count >= num_bits / 2 {
            gamma_bits.push('1');
            epsilon_bits.push('0');
        } else {
            gamma_bits.push('0');
            epsilon_bits.push('1');
        }
    }

    let gamma_str = gamma_bits.iter().join("");
    let epsilon_str = epsilon_bits.iter().join("");
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();

    println!("Problem 03a saw a computed number of {}", gamma * epsilon);
}

pub fn day03_b() {
    let help = day03_helper();
    let line_chars = help.line_chars;
    let num_bits = line_chars[0].len();
    let one_counts = help.one_counts;
    let mut good_oxy: HashSet<String> = HashSet::new();
    let mut good_co2: HashSet<String> = HashSet::new();

    let mut gamma_bits: Vec<char> = vec![];

    for one_count in one_counts.iter() {
        if *one_count >= num_bits / 2 {
            gamma_bits.push('1');
        } else {
            gamma_bits.push('0');
        }
    }

    let sigs = line_chars
        .iter()
        .map(|line| line.iter().join(""))
        .collect_vec();

    for sig in sigs {
        good_oxy.insert(sig.clone());
        good_co2.insert(sig);
    }

    for line in line_chars.iter() {
        for i in 0..num_bits {
            if line[i] != gamma_bits[i] {
                let sig = line.iter().join("");
                good_oxy.remove(&sig);
            }
        }
    }

    for line in line_chars.iter() {
        for i in 0..num_bits {
            if line[i] == gamma_bits[i] {
                let sig = line.iter().join("");
                good_co2.remove(&sig);
            }
        }
    }

    // println!("{:?}", good_oxy);
    // println!("{:?}", good_co2);

    // let oxy_num = usize::from_str_radix(&oxy_readings[0].iter().join(""), 2).unwrap();
    // let co2_num = usize::from_str_radix(&co2_readings[0].iter().join(""), 2).unwrap();

    // println!("Problem 03b saw computed result of {}", oxy_num * co2_num);
}

#[derive(Debug)]
struct Square {
    num: usize,
    checked: bool,
}

impl Square {
    fn num(&self) -> usize {
        self.num
    }

    fn check(&mut self) {
        self.checked = true;
    }

    fn is_checked(&self) -> bool {
        self.checked
    }
}
#[derive(Debug)]
struct Bingo {
    squares: Vec<Vec<Square>>,
    bingoed: bool,
}

impl Bingo {
    // Parse N rows of a bingo card, with each column separated by
    // whitespace. Requires NxN grid.
    fn from(bingo_string: String) -> Self {
        let squares = bingo_string
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|v| Square {
                        num: v.parse::<usize>().unwrap(),
                        checked: false,
                    })
                    .collect_vec()
            })
            .collect_vec();

        Bingo {
            squares: squares,
            bingoed: false,
        }
    }

    // Cross out the number if it exists. Return Some(num) if
    // Bingo is reached. The number returned is the number you hit
    // Bingo on.
    fn cross_out(&mut self, num: usize) -> Option<usize> {
        for row in &mut self.squares {
            for mut square in row {
                if square.num() == num {
                    square.check();
                }
            }
        }

        if !self.bingoed && self.is_bingo() {
            self.bingoed = true;
            Some(num)
        } else {
            None
        }
    }

    fn unseen_sum(&self) -> usize {
        let mut unseen: Vec<usize> = vec![];
        for row in &self.squares {
            for square in row {
                if !square.is_checked() {
                    unseen.push(square.num());
                }
            }
        }
        unseen.iter().sum()
    }

    fn generate_wins(&self) -> Vec<Vec<(usize, usize)>> {
        let mut wins = vec![];
        for i in 0..self.squares.len() {
            let mut row: Vec<(usize, usize)> = vec![];
            for j in 0..self.squares.len() {
                row.push((i, j))
            }
            wins.push(row);
        }

        for i in 0..self.squares.len() {
            let mut col: Vec<(usize, usize)> = vec![];
            for j in 0..self.squares.len() {
                col.push((j, i))
            }
            wins.push(col);
        }

        wins
    }

    fn is_bingo(&self) -> bool {
        for win in self.generate_wins() {
            let hits = win
                .iter()
                .map(|&(x, y)| &self.squares[x][y])
                .filter(|s| s.is_checked())
                .count();

            if hits == self.squares.len() {
                return true;
            }
        }
        false
    }
}

pub fn day04() {
    let mut lines = include_str!("input/day04")
        .lines()
        .map(|l| l.to_string())
        .collect_vec();

    let picks = lines.remove(0);
    let picks = picks
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect_vec();

    lines.remove(0);

    let mut cards: Vec<Bingo> = vec![];
    let mut buf: Vec<String> = vec![];

    for line in &lines {
        if line == "" {
            let card = Bingo::from(buf.join("\n"));
            cards.push(card);
            buf.clear();
            continue;
        }
        buf.push(line.to_string());
    }

    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;

    for pick in picks {
        for mut card in &mut cards {
            if let Some(num) = card.cross_out(pick) {
                if first.is_none() {
                    first = Some(card.unseen_sum() * pick);
                    continue;
                }

                let calc = card.unseen_sum() * pick;
                if calc > 0 {
                    last = Some(calc);
                }
            }
        }
    }

    println!("Problem 04a bingo calc is {}", first.unwrap());
    println!("Problem 04b bingo calc is {}", last.unwrap());
}

pub fn day05_a() {
    let lines = include_str!("input/day05")
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_vec());

    let mut grid = vec![0usize; 1_000_000];

    for line in lines {
        let start = line[0]
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect_vec();

        let end = line[2]
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect_vec();

        let (x0, y0) = (start[0], start[1]);
        let (x1, y1) = (end[0], end[1]);
        let (x0, x1) = (x0.min(x1), x0.max(x1));
        let (y0, y1) = (y0.min(y1), y0.max(y1));

        for y in y0..=y1 {
            for x in x0..=x1 {
                // If it's a straight line, you will share either a
                // common x or a common y.
                if x0 == x1 || y0 == y1 {
                    grid[((1000 * y) + x)] += 1;
                }
            }
        }
    }

    println!("Problem 05a is {}", grid.iter().filter(|n| **n > 1).count());
}

pub fn day05_b() {
    let lines = //include_str!("input/day05")
 "0,9 -> 5,9
 8,0 -> 0,8
 9,4 -> 3,4
 2,2 -> 2,1
 7,0 -> 7,4
 6,4 -> 2,0
 0,9 -> 2,9
 3,4 -> 1,4
 0,0 -> 8,8
 5,5 -> 8,2"       
    .lines()
        .map(|l| l.split_ascii_whitespace().collect_vec());

    let mut grid = vec![0usize; 1_000_000];

    for line in lines {
        let start = line[0]
            .split(",")
            .map(|c| c.parse::<isize>().unwrap())
            .collect_vec();

        let end = line[2]
            .split(",")
            .map(|c| c.parse::<isize>().unwrap())
            .collect_vec();

        let (x0, y0) = (start[0], start[1]);
        let (x1, y1) = (end[0], end[1]);

        // println!("{},{} to {},{}", x0, y0, x1, y1);
        {
            let (fx0, fx1) = (x0.min(x1), x0.max(x1));
            let (fy0, fy1) = (y0.min(y1), y0.max(y1));

            if (x1 - x0).abs() == (y1 - y0).abs() {
                let mut x_p = x0;
                for x in x0..=x1 {
                    let y_p;
                    if y1 > y0 {
                        y_p = y0 + x_p;
                    } else {
                        y_p = y0 - x_p;
                    }

                    if x1 > x0 {
                        x_p = x0 + x_p - fx0;
                    } else {
                        x_p = x0 - y_p;
                    }

                    // println!("Drawing {},{}", x_p, y_p);

                    let square = ((10 * y_p) + x_p) as usize;
                    grid[square] += 1;
                }
            }
        }

        {
            let (x0, x1) = (x0.min(x1), x0.max(x1));
            let (y0, y1) = (y0.min(y1), y0.max(y1));

            for y in y0..=y1 {
                for x in x0..=x1 {
                    // If it's a straight line, you will share either a
                    // common x or a common y.
                    // println!("Drawing {},{}", x, y);
                    let square = ((10 * y) + x) as usize;
                    if x0 == x1 || y0 == y1 {
                        grid[square] += 1;
                    }
                }
            }
        }
    }

    for y in 0..=9 {
        for x in 0..=9 {
            let c = match grid[(y * 10) + x] {
                0 => '.',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => '?',
            };
            // print!("{}", c);
        }
        // println!();
    }

    println!("Problem 05b is {}", grid.iter().filter(|n| **n > 1).count());
}

struct Lanternfish {
    age: usize,
}

impl Lanternfish {
    fn new() -> Self {
        Self { age: 8 }
    }

    fn age(&self) -> usize {
        self.age
    }

    fn with_age(age: usize) -> Self {
        Self { age: age }
    }

    fn tick(&mut self) -> Option<Self> {
        if self.age == 0 {
            self.age = 6;
            return Some(Self::new());
        } else {
            self.age -= 1;
        }

        None
    }
}

pub fn day06() {
    let init_fish = include_str!("input/day06")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    // My old solution broke down around day 160 and 500M
    // or so fish structs. I had to think of an easier way
    // to represent the fish without exponential memory
    // growth.
    //
    // The solution is to realize that you can just create
    // a set of counters, one per fish lifetime. When you
    // "age" a fish from age 5 to age 4 for example, you just
    // decrement from the count of 5-fish and add to the count
    // of 4-fish.
    //
    // So we just keep track of 9 fish lifetimes and do a
    // bunch of adds and subtracts for every day tick.

    let mut fish_counts: Vec<usize> = vec![0; 9];
    for fish in init_fish {
        fish_counts[fish] += 1;
    }

    for day in 1..=256 {
        let num_new = fish_counts[0];
        let copy = fish_counts.clone();
        for age in 1..=8 {
            fish_counts[age - 1] = copy[age];
        }
        fish_counts[6] += num_new;
        fish_counts[8] = num_new;

        if day == 80 {
            println!("Problem 06a is {}", fish_counts.iter().sum::<usize>());
        }

        if day == 256 {
            println!("Problem 06b is {}", fish_counts.iter().sum::<usize>());
        }
    }
}

pub fn day07_a() {
    let starting_positions = include_str!("input/day07")
        .split(",")
        .map(|v| v.parse::<isize>().unwrap())
        .collect_vec();

    let unique_positions = starting_positions.iter().unique().collect_vec();

    let mut min_pos: (isize, usize) = (-1, usize::MAX);
    for upos in &unique_positions {
        let mut cost = 0usize;
        for pos in &starting_positions {
            cost += (*upos - pos).abs() as usize;
        }
        if cost < min_pos.1 {
            min_pos = (**upos, cost);
        }
    }

    println!("Problem 07a is {}", min_pos.1);
}

pub fn day07_b() {
    let starting_positions = include_str!("input/day07")
        .split(",")
        .map(|v| v.parse::<isize>().unwrap())
        .collect_vec();

    let unique_positions = starting_positions.iter().unique().collect_vec();

    let mut min_pos: (isize, usize) = (-1, usize::MAX);
    for upos in &unique_positions {
        let mut cost = 0usize;
        for pos in &starting_positions {
            let travel = (*upos - pos).abs() as usize;
            cost += (1..=travel).fold(0, |acc, x| acc + x);
        }
        if cost < min_pos.1 {
            min_pos = (**upos, cost);
        }
    }

    println!("Problem 07b is {}", min_pos.1);
}

pub fn day08_a() {
    let readings = include_str!("input/day08")
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|s| s.split_ascii_whitespace().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let mut easy_digit_count = 0usize;
    for reading in &readings {
        let (signals, digits) = (&reading[0], &reading[1]);
        easy_digit_count += digits
            .iter()
            .filter(|s| match s.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count();
    }

    println!("Problem 08a is {}", easy_digit_count);

    let mut num_lengths: HashMap<usize, usize> = HashMap::from([(1, 2), (4, 4), (7, 3), (8, 7)]);

    for reading in &readings {
        let (signals, digits) = (&reading[0], &reading[1]);
    }
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
