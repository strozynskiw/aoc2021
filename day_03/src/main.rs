use std::collections::btree_map::Values;
use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<u8>>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '1' => 1,
                    '0' => 0,
                    _ => panic!("Incorrect char"),
                })
                .collect()
        })
        .collect()
}

fn part_1(input: &Generated) -> i32 {
    let mut values: Vec<usize> = Vec::new();
    values.resize(input[0].len(), 0);

    input.iter().for_each(|v| {
        v.iter()
            .enumerate()
            .for_each(|(n, v)| values[n] += *v as usize)
    });

    let mut gamma = 0;
    let mut epsilon = 0;

    values.iter().for_each(|v| {
        gamma <<= 1;
        epsilon <<= 1;
        match *v > input.len() / 2 {
            true => {
                gamma += 1;
            }
            false => {
                epsilon += 1;
            }
        };
    });

    gamma * epsilon
}

fn part_2(input: &Generated) -> i32 {
    let mut oxy = input.clone();
    let mut co2 = input.clone();

    let mut oxy_val = 0;
    let mut co2_val = 0;

    //I will clean that up one day...
    for i in 0..input[0].len() {
        let mut temp1: Generated = Vec::new();
        let mut temp2: Generated = Vec::new();

        co2.iter().for_each(|item| {
            if item[i] == 1 {
                temp1.push(item.clone())
            } else {
                temp2.push(item.clone())
            }
        });

        match temp1.len() < temp2.len() {
            true => co2 = temp1,
            false => co2 = temp2,
        }

        if co2.len() == 1 {
            co2[0].iter().for_each(|v| {
                co2_val <<= 1;
                co2_val += *v as i32;
            });
            break;
        }
    }

    for i in 0..input[0].len() {
        let mut temp1: Generated = Vec::new();
        let mut temp2: Generated = Vec::new();

        oxy.iter().for_each(|item| {
            if item[i] == 1 {
                temp1.push(item.clone())
            } else {
                temp2.push(item.clone())
            }
        });

        match temp1.len() >= temp2.len() {
            true => oxy = temp1,
            false => oxy = temp2,
        }

        if oxy.len() == 1 {
            oxy[0].iter().for_each(|v| {
                oxy_val <<= 1;
                oxy_val += *v as i32;
            });
            break;
        }
    }

    oxy_val * co2_val
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
            20450,
            part_1(&generate("000000000001\n000000000100\n000000000101\n"))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            230,
            part_2(&generate(
                "00100\n
        11110\n
        10110\n
        10111\n
        10101\n
        01111\n
        00111\n
        11100\n
        10000\n
        11001\n
        00010\n
        01010\n"
            ))
        );
    }
}
