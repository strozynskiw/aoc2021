use std::fs;
use std::time::Instant;

const BOARD_SIZE: usize = 5;

type Board = Vec<Vec<usize>>;

struct Generated {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

fn generate(input: &str) -> Generated {
    let mut input = input.split("\n\n");

    let numbers = input.next().unwrap();

    let numbers = numbers
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect::<Vec<usize>>();

    let boards = input
        .map(|b| {
            b.split('\n')
                .map(|line| {
                    line.trim()
                        .split_whitespace()
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Board>()
        })
        .collect::<Vec<Board>>();

    Generated { numbers, boards }
}

fn board_wins(board: &[[bool; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    let full_row = board.iter().any(|r| r == &[true; BOARD_SIZE]);
    let full_col = (0..5).any(|col| {
        board[0][col] && board[1][col] && board[2][col] && board[3][col] && board[4][col]
    });
    full_row || full_col
}

struct BoardResult {
    n: usize,
    score: usize,
}

fn get_score(numbers: &Vec<usize>, board: &Board) -> Option<BoardResult> {
    let mut sum = board.iter().flatten().sum::<usize>();
    let mut hit_board = [[false; BOARD_SIZE]; BOARD_SIZE];

    for (n, number) in numbers.iter().enumerate() {
        for (i, row) in board.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if number == v {
                    hit_board[i][j] = true;
                    sum -= number;
                    if board_wins(&hit_board) {
                        return Some(BoardResult {
                            n,
                            score: sum * number,
                        });
                    }
                }
            }
        }
    }

    None
}

fn part_1(input: &Generated) -> usize {
    let results: Vec<BoardResult> = input
        .boards
        .iter()
        .map(|b| get_score(&input.numbers, b).expect("No result"))
        .collect();

    results.iter().min_by(|a, b| a.n.cmp(&b.n)).unwrap().score
}

fn part_2(input: &Generated) -> usize {
    let results: Vec<BoardResult> = input
        .boards
        .iter()
        .map(|b| get_score(&input.numbers, b).expect("No result"))
        .collect();

    results.iter().max_by(|a, b| a.n.cmp(&b.n)).unwrap().score
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
            4512,
            part_1(&generate(
                r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            ))
        )
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            1924,
            part_2(&generate(
                r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            ))
        )
    }
}
