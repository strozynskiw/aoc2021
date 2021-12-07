use std::fs;
use std::time::Instant;

type Generated = Vec<i64>;

fn generate(input: &str) -> Generated {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    lines[0]
        .split(',')
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

fn part_1(input: &Generated) -> i64 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    (*min..*max)
        .map(|alignment| input.iter().map(|crab| i64::abs(alignment - crab)).sum())
        .min()
        .unwrap()
}

fn part_2(input: &Generated) -> i64 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    (*min..*max)
        .map(|alignment| {
            input
                .iter()
                .map(|crab| {
                    let n = i64::abs(alignment - crab);
                    (n * (n + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
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
        assert_eq!(37, part_1(&generate("16,1,2,0,4,2,7,1,2,14")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(168, part_2(&generate("16,1,2,0,4,2,7,1,2,14")));
    }
}
