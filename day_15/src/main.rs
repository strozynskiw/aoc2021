use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<u8>>;

#[derive(Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    point: (usize, usize),
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        (usize::MAX - self.cost).cmp(&(usize::MAX - other.cost))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect()
}

fn get_neighbors(size: (usize, usize), position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (x, y) in neighbors {
        if (x + position.0 as i32) >= 0
            && (x + position.0 as i32) < size.0 as i32
            && (y + position.1 as i32) >= 0
            && (y + position.1 as i32) < size.1 as i32
        {
            result.push((
                (position.0 as i32 + x) as usize,
                (position.1 as i32 + y) as usize,
            ));
        }
    }

    result
}

fn find(input: &Generated) -> usize {
    let mut paths: BinaryHeap<Path> = BinaryHeap::new();
    let end = (input.len() - 1, input[0].len() - 1);
    let size = (input.len(), input[0].len());
    let mut risks: HashMap<(usize, usize), usize> = HashMap::new();

    paths.push(Path {
        cost: 0,
        point: (0, 0),
    });

    while let Some(Path { cost, point }) = paths.pop() {
        if point == end {
            return cost;
        }

        for n in get_neighbors(size, &point) {
            let new_cost = cost + input[n.0][n.1] as usize;

            if new_cost < *risks.get(&n).unwrap_or(&usize::MAX) {
                risks.insert(n, new_cost);
                paths.push(Path {
                    cost: new_cost,
                    point: n,
                });
            }
        }
    }
    0
}

fn part_1(input: &Generated) -> usize {
    find(input)
}

fn expand(map: &Generated) -> Vec<Vec<u8>> {
    let mut all = vec![];
    let mut rows = vec![];
    for row in map {
        let mut new_row = vec![];
        for offset in 0..5 {
            for c in row {
                let mut v = *c + offset;
                if v > 9 {
                    v -= 9;
                }
                new_row.push(v);
            }
        }
        rows.push(new_row);
    }
    for offset in 0..5 {
        for row in &rows {
            let mut new_row = row.clone();
            for v in &mut new_row {
                let mut new_v = *v + offset;
                if new_v > 9 {
                    new_v -= 9;
                }
                *v = new_v;
            }
            all.push(new_row);
        }
    }
    all
}

fn part_2(input: &Generated) -> usize {
    let new_input = expand(&input);
    find(&new_input)
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
            40,
            part_1(&generate(
                "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(&generate("")));
    }
}
