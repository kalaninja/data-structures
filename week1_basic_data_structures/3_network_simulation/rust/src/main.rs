use std::collections::VecDeque;
use std::str::FromStr;
use std::fmt::Debug;

struct Packet {
    time: u32,
    length: u32,
}

impl Packet {
    fn new(time: u32, length: u32) -> Self {
        Packet { time, length }
    }
}

fn main() {
    let (s, n) = read_line::<usize>();

    let packets =
        (0..n).fold(Vec::with_capacity(n), |mut acc, _| {
            match read_line() { (a, b) => acc.push(Packet::new(a, b)) };
            acc
        });

    simulate(packets, s).iter()
        .map(|x| x.map_or("-1".to_string(), |y| y.to_string()))
        .for_each(|x| println!("{}", x));
}

fn simulate(packets: Vec<Packet>, buffer_size: usize) -> Vec<Option<u32>> {
    let mut start_times = Vec::with_capacity(packets.len());
    let mut buffer = VecDeque::with_capacity(buffer_size);

    for packet in packets {
        while buffer.front().map_or(false, |&x| x <= packet.time) {
            buffer.pop_front();
        }

        if buffer.len() < buffer_size {
            let start_time = buffer.back().map_or(
                packet.time,
                |&x| packet.time.max(x));
            start_times.push(Some(start_time));
            buffer.push_back(start_time + packet.length);
        } else {
            start_times.push(None);
        }
    }

    start_times
}

fn read_line<T: FromStr + Copy>() -> (T, T) where <T as FromStr>::Err: Debug {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let vec = input.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<T>>();

    (vec[0], vec[1])
}

// #[cfg(test)]
// mod tests;