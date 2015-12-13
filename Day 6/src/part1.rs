use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::fs::File;

fn toggle(lights: &mut [[bool; 1000]; 1000], x1: usize, x2: usize, y1: usize, y2: usize) {
    for x in x1..(x2+1) {
        for y in y1..(y2+1) {
            lights[x][y] ^= true;
        }
    }
}

fn set(lights: &mut [[bool; 1000]; 1000], on: bool, x1: usize, x2: usize, y1: usize, y2: usize) {
    for x in x1..(x2+1) {
        for y in y1..(y2+1) {
            lights[x][y] = on;
        }
    }
}

fn count(lights: &[[bool; 1000]; 1000]) -> u32 {
    let mut n: u32 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if lights[x][y] {
                n += 1;
            }
        }
    }
    return n;
}

pub fn part1() {
    let re = Regex::new(r"(?m)^(?P<instruction>turn on|turn off|toggle)\s(?P<x1>\d+),(?P<y1>\d+)\sthrough\s(?P<x2>\d+),(?P<y2>\d+)$").unwrap();
    let mut lights = [[false; 1000]; 1000];

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
            "toggle"   => toggle(&mut lights, x1, x2, y1, y2),
            "turn on"  => set(&mut lights, true, x1, x2, y1, y2),
            "turn off" => set(&mut lights, false, x1, x2, y1, y2),
            _ => (),
        }
    }
    println!("After following Santa's instructions {} lights will be on", count(&lights));
}
