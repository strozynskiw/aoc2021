use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;
use std::{fs, panic};

enum Code {
    Up,
    Down,
    Forward,
    Unknown,
}
struct Operation {
    code: Code,
    value: i32,
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let code: Code = match parts[0] {
            "up" => Code::Up,
            "down" => Code::Down,
            "forward" => Code::Forward,
            _ => Code::Unknown,
        };

        let value = parts[1].parse::<i32>()?;
        Ok(Operation { code, value })
    }
}

type Generated = Vec<Operation>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Operation::from_str(l).unwrap())
        .collect::<Vec<Operation>>()
}

struct Position(i32, i32);

fn part_1(input: &Generated) -> i32 {
    let mut position = Position(0, 0);
    input.iter().for_each(|i| match i.code {
        Code::Forward => position.0 += i.value,
        Code::Up => position.1 -= i.value,
        Code::Down => position.1 += i.value,
        Code::Unknown => panic!("Incorrect command"),
    });
    position.1 * position.0
}

fn part_2(input: &Generated) -> i32 {
    let mut position = Position(0, 0);
    let mut aim = 0;
    input.iter().for_each(|i| match i.code {
        Code::Forward => {
            position.0 += i.value;
            position.1 += i.value * aim
        }
        Code::Up => aim -= i.value,
        Code::Down => aim += i.value,
        Code::Unknown => panic!("Incorrect command"),
    });
    position.1 * position.0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let data = generate(&content);

    let res1_start = Instant::now();
    let res1 = part_1(&data);
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&data);
    let res2_stop = Instant::now();

    print!(
        "Result1: {}\nResolved in: {:?}\n",
        res1,
        res1_stop.duration_since(res1_start)
    );
    print!(
        "Result2: {}\nResolved in: {:?}\n",
        res2,
        res2_stop.duration_since(res2_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(
            150,
            part_1(&generate(
                &"forward 5\n
        down 5\n
        forward 8\n
        up 3\n
        down 8\n
        forward 2\n"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            900,
            part_2(&generate(
                &"forward 5\n
        down 5\n
        forward 8\n
        up 3\n
        down 8\n
        forward 2\n"
            ))
        );
    }
}
