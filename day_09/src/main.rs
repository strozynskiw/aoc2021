use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<Cell>>;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Cell {
    Flooded,
    Dry(i32),
}

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| Cell::Dry(c as i32 - 48)).collect())
        .collect()
}

fn get_minimas(input: &Generated) -> Vec<(usize, usize)> {
    let mut minimas: Vec<(usize, usize)> = Vec::new();
    for x in 0..input[0].len() as i32 {
        for y in 0..input.len() as i32 {
            let mut points: Vec<Cell> = Vec::new();
            let val = input[y as usize][x as usize];

            if x - 1 >= 0 {
                points.push(input[y as usize][x as usize - 1]);
            }
            if x + 1 < input[0].len() as i32 {
                points.push(input[y as usize][x as usize + 1]);
            }
            if y - 1 >= 0 {
                points.push(input[y as usize - 1][x as usize]);
            }
            if y + 1 < input.len() as i32 {
                points.push(input[y as usize + 1][x as usize]);
            }

            if points.iter().any(|p| p <= &val) {
                continue;
            } else {
                minimas.push((y as usize, x as usize));
            }
        }
    }
    minimas
}

fn part_1(input: &Generated) -> i32 {
    let minimas = get_minimas(input);

    minimas.iter().map(|m| match input[m.0][m.1] {
        Cell::Dry(v) => v,
        _ => 0,
        } + 1).sum()
}

fn flood_fill(input: &mut Generated, start: &(usize, usize)) -> usize {
    match input[start.0][start.1] {
        Cell::Dry(v) if v != 9 => {
            input[start.0][start.1] = Cell::Flooded;
            let mut result = 1;
            if start.0 as i32 - 1 >= 0 {
                result += flood_fill(input, &(start.0 - 1, start.1));
            }
            if start.0 + 1 < input.len() {
                result += flood_fill(input, &(start.0 + 1, start.1));
            }
            if start.1 as i32 - 1 >= 0 {
                result += flood_fill(input, &(start.0, start.1 - 1));
            }
            if start.1 + 1 < input[0].len() {
                result += flood_fill(input, &(start.0, start.1 + 1));
            }
            result
        },
        _ => 0,
    }
}

fn part_2(input: &Generated) -> usize {
    let minimas = get_minimas(input);
    let mut results = minimas
        .iter()
        .map(|position| flood_fill(&mut input.clone(), position))
        .collect::<Vec<usize>>();
    results.sort_unstable();
    results.iter().rev().take(3).product()
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
            15,
            part_1(&generate(
                "2199943210
        3987894921
        9856789892
        8767896789
        9899965678"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            1134,
            part_2(&generate(
                "2199943210
        3987894921
        9856789892
        8767896789
        9899965678"
            ))
        );
    }
}
