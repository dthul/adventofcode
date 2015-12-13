use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::fs::File;

fn inc(lights: &mut [[u16; 1000]; 1000], amount: u16, x1: usize, x2: usize, y1: usize, y2: usize) {
    for x in x1..(x2+1) {
        for y in y1..(y2+1) {
            lights[x][y] += amount;
        }
    }
}

fn dec(lights: &mut [[u16; 1000]; 1000], amount: u16, x1: usize, x2: usize, y1: usize, y2: usize) {
    for x in x1..(x2+1) {
        for y in y1..(y2+1) {
            if lights[x][y] > amount {
                lights[x][y] -= amount;
            }
            else {
                lights[x][y] = 0;
            }
        }
    }
}

fn count(lights: &[[u16; 1000]; 1000]) -> u32 {
    let mut n: u32 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            n += lights[x][y] as u32;
        }
    }
    return n;
}

pub fn part2() {
    let re = Regex::new(r"(?m)^(?P<instruction>turn on|turn off|toggle)\s(?P<x1>\d+),(?P<y1>\d+)\sthrough\s(?P<x2>\d+),(?P<y2>\d+)$").unwrap();
    let mut lights = [[0u16; 1000]; 1000];

    let mut file = match File::open("input.txt") {
        Err(why) => panic!("couldn't open input: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut instructions = String::new();
    match file.read_to_string(&mut instructions) {
        Err(why) => panic!("couldn't read input: {}", Error::description(&why)),
        Ok(_) => (),
    }

    for cap in re.captures_iter(&instructions) {
        let x1 = cap.name("x1").unwrap().parse::<usize>().unwrap();
        let x2 = cap.name("x2").unwrap().parse::<usize>().unwrap();
        let y1 = cap.name("y1").unwrap().parse::<usize>().unwrap();
        let y2 = cap.name("y2").unwrap().parse::<usize>().unwrap();
        match cap.name("instruction").unwrap() {
            "toggle"   => inc(&mut lights, 2, x1, x2, y1, y2),
            "turn on"  => inc(&mut lights, 1, x1, x2, y1, y2),
            "turn off" => dec(&mut lights, 1, x1, x2, y1, y2),
            _ => (),
        }
    }
    println!("After following Santa's instructions the total brightness will be {}", count(&lights));
}
