use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::fs::File;

#[derive(Clone, Debug)]
enum Wire {
    Value(u16),
    VAnd(u16, String),
    And(String, String),
    Or(String, String),
    LShift(String, u8),
    RShift(String, u8),
    Not(String),
    Id(String),
}

fn compute(w: &str, h: &mut HashMap<String, Wire>) -> u16 {
    let wire = (*h.get(w).unwrap()).clone();
    return match wire {
        Wire::Value(v)            => v,
        Wire::VAnd(v1, w2)    => {
            let v = v1 & compute(&w2, h);
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
        Wire::And(w1, w2)    => {
            let v =  compute(&w1, h) & compute(&w2, h);
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
        Wire::Or(w1, w2)    => {
            let v =  compute(&w1, h) | compute(&w2, h);
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
        Wire::LShift(w1, v1)    => {
            let v = compute(&w1, h) << v1;
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
        Wire::RShift(w1, v1)    => {
            let v = compute(&w1, h) >> v1;
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
        Wire::Not(w1)    => {
            let v = compute(&w1, h) ^ 0xFFFF;
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
        Wire::Id(w1)    => {
            let v = compute(&w1, h);
            h.insert(w.to_string(), Wire::Value(v));
            v
        },
    }
}

fn read(instructions: &str, graph: &mut HashMap<String, Wire>) {
    let re = Regex::new(r"(?m)^(?P<in1>[a-z]+|[0-9]+)?(?P<op> | AND | OR | LSHIFT | RSHIFT |NOT )(?P<in2>[a-z]+|[0-9]+)?\s?-> (?P<out>[a-z]+)$").unwrap();

    for cap in re.captures_iter(instructions) {
        let in1 = cap.name("in1").unwrap_or("");
        let in2 = cap.name("in2").unwrap_or("");
        let op = cap.name("op").unwrap_or("");
        let out = cap.name("out").unwrap();
        // let w2 = graph.get(&in2);
        let tmp: Option<Wire> = match op {
            " "        => match in1.parse::<u16>() {
                Ok(v)  => graph.insert(out.to_string(), Wire::Value(v)),
                Err(_) => graph.insert(out.to_string(), Wire::Id(in1.to_string())),
            },
            " AND "    => match in1.parse::<u16>() {
                Ok(v)  => graph.insert(out.to_string(), Wire::VAnd(v, in2.to_string())),
                Err(_) => graph.insert(out.to_string(), Wire::And(in1.to_string(), in2.to_string())),
            },
            " OR "     => graph.insert(out.to_string(), Wire::Or(in1.to_string(), in2.to_string())),
            " LSHIFT " => graph.insert(out.to_string(), Wire::LShift(in1.to_string(), in2.parse::<u8>().unwrap())),
            " RSHIFT " => graph.insert(out.to_string(), Wire::RShift(in1.to_string(), in2.parse::<u8>().unwrap())),
            "NOT "     => graph.insert(out.to_string(), Wire::Not(in2.to_string())),
            _          => None,
        };
        match tmp {
            None => (),
            Some(_) => panic!("Encountered duplicate output wire {}", out),
        }
    }
}

pub fn main() {
    let mut graph: HashMap<String, Wire> = HashMap::new();

    let mut file = match File::open("input.txt") {
        Err(why) => panic!("couldn't open input: {}", Error::description(&why)),
        Ok(file) => file,
    };

    let mut instructions = String::new();
    match file.read_to_string(&mut instructions) {
        Err(why) => panic!("couldn't read input: {}", Error::description(&why)),
        Ok(_) => (),
    }

    read(&instructions, &mut graph);

    let mut a = compute("a", &mut graph);

    graph.clear();
    read(&instructions, &mut graph);

    graph.insert("b".to_string(), Wire::Value(a));
    a = compute("a", &mut graph);
    println!("After helping little Bobby Tables assemble the circuit the signal on wire a is {}", a);
}
