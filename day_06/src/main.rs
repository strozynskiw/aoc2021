use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

type Generated = VecDeque<u128>;

fn generate(input: &str) -> Generated {
    let values = input
        .lines()
        .take(1)
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();
    let mut collection = [0; 9];
    values[0].iter().for_each(|&v| collection[v as usize] += 1);
    VecDeque::from(collection)
}

fn part_1(input: &mut Generated) -> u128 {
    for _ in 0..80 {
        let value = input.pop_front().unwrap();
        input[6] += value;
        input.push_back(value);
    }

    input.iter().sum::<u128>()
}

fn part_2(input: &mut Generated) -> u128 {
    for _ in 0..256 {
        let value = input.pop_front().unwrap();
        input[6] += value;
        input.push_back(value);
    }

    input.iter().sum::<u128>()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let res1_start = Instant::now();
    let res1 = part_1(&mut generate(&content));
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&mut generate(&content));
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
        assert_eq!(5934, part_1(&mut generate("3,4,3,1,2")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(26984457539, part_2(&mut generate("3,4,3,1,2")));
    }
}
