use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Generated {
    points: HashSet<(usize, usize)>,
    folds: Vec<(char, usize)>,
}

fn print_board(size: &(usize, usize), points: &HashSet<(usize, usize)>) {
    for i in 0..size.1 {
        for j in 0..size.0 {
            if let Some(_) = points.get(&(j, i)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn generate(input: &str) -> Generated {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let points = parts[0]
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<HashSet<(usize, usize)>>();

    let folds = parts[1]
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split('=');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .map(|l| {
            if l.0.contains('x') {
                ('x', l.1)
            } else {
                ('y', l.1)
            }
        })
        .collect::<Vec<(char, usize)>>();

    Generated { points, folds }
}

fn find_sheet_size(folds: &Vec<(char, usize)>) -> (usize, usize) {
    let first_x = folds.iter().find(|c| c.0 == 'x').expect("X Fold not found");
    let first_y = folds.iter().find(|c| c.0 == 'y').expect("Y Fold not found");

    (first_x.1 * 2 + 1, first_y.1 * 2 + 1)
}

fn make_fold(
    fold: (char, usize),
    size: &(usize, usize),
    points: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let new_size = match fold.0 {
        'x' => (fold.1, size.1),
        'y' => (size.0, fold.1),
        _ => panic!("Incorrect fold axis"),
    };

    let to_be_folded = points
        .iter()
        .filter(|pair| pair.0 > new_size.0 || pair.1 > new_size.1)
        .copied()
        .collect::<Vec<(usize, usize)>>();

    for pair in to_be_folded {
        let new_point = match fold.0 {
            'x' => (fold.1 - (pair.0 - fold.1), pair.1),
            'y' => (pair.0, (fold.1 - (pair.1 - fold.1))),
            _ => panic!("Incorrect fold axis"),
        };

        points.insert(new_point);
        points.remove(&pair);
    }

    new_size
}

fn part_1(input: &Generated) -> usize {
    let mut points = input.points.clone();
    let size = find_sheet_size(&input.folds);
    make_fold(input.folds[0], &size, &mut points);

    points.len()
}

fn part_2(input: &Generated) -> () {
    let mut points = input.points.clone();

    let mut size = find_sheet_size(&input.folds);

    for fold in &input.folds {
        size = make_fold(*fold, &size, &mut points);
    }
    print_board(&size, &points);
}

fn main() {
    let content = fs::read_to_string("input2").expect("file not found");

    let data = generate(&content);

    let res1_start = Instant::now();
    let res1 = part_1(&data);
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let _ = part_2(&data);
    let res2_stop = Instant::now();

    print!(
        "Result1: {}\nResolved in: {:?}\n",
        res1,
        res1_stop.duration_since(res1_start)
    );
    print!(
        "Result2: {}\nResolved in: {:?}\n",
        0,
        res2_stop.duration_since(res2_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(
            17,
            part_1(&generate(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            ))
        );
    }
}
