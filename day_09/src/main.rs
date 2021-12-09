use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<i32>>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c as i32 - 48).collect())
        .collect()
}

fn get_minimas(input: &Generated) -> Vec<(usize, usize)> {
    let mut minimas: Vec<(usize, usize)> = Vec::new();
    for x in 0..input[0].len() as i32 {
        for y in 0..input.len() as i32 {
            let mut points: Vec<i32> = Vec::new();
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

            if points.iter().any(|&p| p <= val) {
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

    minimas.iter().map(|m| input[m.0][m.1] + 1).sum()
}

#[derive(PartialEq)]
enum Cell {
    Edge,
    Flooded,
    Dry(u8),
}

fn find_edge(board: &Vec<Vec<Cell>>) -> Option<(usize, usize)> {
    for x in 0..board[0].len() {
        for y in 0..board.len() {
            if board[y][x] == Cell::Edge {
                return Some((y, x));
            }
        }
    }
    None
}

fn flood_fill(input: &Generated, start: &(usize, usize)) -> usize {
    let mut board: Vec<Vec<Cell>> = input
        .iter()
        .map(|row| row.iter().map(|i| Cell::Dry(*i as u8)).collect())
        .collect();

    board[start.0][start.1] = Cell::Edge;

    while let Some(edge) = find_edge(&board) {
        board[edge.0][edge.1] = Cell::Flooded;

        if edge.0 as i32 - 1 >= 0 {
            match board[edge.0 - 1][edge.1] {
                Cell::Dry(v) if v != 9 => board[edge.0 - 1][edge.1] = Cell::Edge,
                _ => (),
            }
        }
        if edge.0 + 1 < board.len() {
            match board[edge.0 + 1][edge.1] {
                Cell::Dry(v) if v != 9 => board[edge.0 + 1][edge.1] = Cell::Edge,
                _ => (),
            }
        }
        if edge.1 as i32 - 1 >= 0 {
            match board[edge.0][edge.1 - 1] {
                Cell::Dry(v) if v != 9 => board[edge.0][edge.1 - 1] = Cell::Edge,
                _ => (),
            }
        }
        if edge.1 + 1 < input[0].len() {
            match board[edge.0][edge.1 + 1] {
                Cell::Dry(v) if v != 9 => board[edge.0][edge.1 + 1] = Cell::Edge,
                _ => (),
            }
        }
    }

    board
        .iter()
        .map(|row| row.iter().filter(|&c| *c == Cell::Flooded).count())
        .sum()
}

fn part_2(input: &Generated) -> usize {
    let minimas = get_minimas(input);
    let mut results = minimas
        .iter()
        .map(|position| flood_fill(input, position))
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
