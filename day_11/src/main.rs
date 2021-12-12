use std::fs;
use std::time::Instant;

#[derive(PartialEq, Debug)]
enum Cell {
    Normal(u32),
    Fired,
}
type Generated = Vec<Vec<Cell>>;

fn print_board(data: &Generated) {
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            match data[x][y] {
                Cell::Normal(v) => print!("{}", &v),
                Cell::Fired => print!("F"),
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| Cell::Normal(c as u32 - 48)).collect())
        .collect()
}

fn get_neighbours(data: &Generated, position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    let x_max = data.len() as i32;
    let y_max = data[0].len() as i32;

    for x in -1..=1 {
        for y in -1..=1 {
            if (x + position.0 as i32) >= 0
                && (x + position.0 as i32) < x_max
                && (y + position.1 as i32) >= 0
                && (y + position.1 as i32) < y_max
            {
                result.push((
                    (position.0 as i32 + x) as usize,
                    (position.1 as i32 + y) as usize,
                ));
            }
        }
    }

    result
}

fn increese_energy(data: &mut Generated, position: &(usize, usize)) {
    match data[position.0][position.1] {
        Cell::Normal(v) if v < 9 => {
            data[position.0][position.1] = Cell::Normal(v + 1);
        }
        Cell::Normal(v) if v == 9 => {
            data[position.0][position.1] = Cell::Fired;
            for n in get_neighbours(data, position) {
                increese_energy(data, &n);
            }
        }
        _ => (),
    }
}

fn make_step(data: &mut Generated) -> u32 {
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            increese_energy(data, &(x, y));
        }
    }

    let mut fired = 0;
    data.iter_mut().for_each(|i| {
        i.iter_mut().for_each(|j| {
            if *j == Cell::Fired {
                fired += 1;
                *j = Cell::Normal(0);
            }
        })
    });

    fired
}

fn part_1(input: &mut Generated) -> u32 {
    (0..100).map(|_| make_step(input)).sum()
}

fn part_2(input: &mut Generated) -> u32 {
    let mut step = 0;
    let num_of_cells = input.len() * input[0].len();

    loop {
        step += 1;
        if make_step(input) as usize == num_of_cells {
            break;
        }
    }

    step
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
        assert_eq!(
            1656,
            part_1(&mut generate(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            ))
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            195,
            part_2(&mut generate(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            ))
        );
    }
}
