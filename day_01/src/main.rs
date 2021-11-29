use std::fs;
use std::time::Instant;

type Generated = Vec<i32>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part_1(input: &Generated) -> i32 {
    input.windows(2).filter(|w| w[0] < w[1]).count() as i32
}


fn part_2(input: &Generated) -> i32 {
    let sums: Vec<i32> = input.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    sums.windows(2).filter(|w| w[0] < w[1]).count() as i32
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

    print!("Result1: {}\nResolved in: {:?}\n", res1, res1_stop.duration_since(res1_start));
    print!("Result2: {}\nResolved in: {:?}\n", res2, res2_stop.duration_since(res2_start));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(&[5,6,5,6].to_vec()))
    }
    #[test]
    fn test_part_2() {
        assert_eq!(5, part_1(&[607,618,618,617,647,716,769,792].to_vec()))
    }
}
