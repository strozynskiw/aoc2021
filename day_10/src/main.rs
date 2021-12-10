use core::panic;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<char>>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn find_first_incorrect_character(data: &Vec<char>) -> Option<char> {
    let opening_elements = ['<', '(', '{', '['];
    let map: HashMap<char, char> = [('<', '>'), ('(', ')'), ('[', ']'), ('{', '}')].into();

    let mut stack: Vec<char> = Vec::new();

    for c in data {
        if opening_elements.contains(&c) {
            stack.push(*c);
        } else {
            if let Some(stack) = stack.pop() {
                if *c != *map.get(&stack).expect("Incorrect mapping") {
                    return Some(*c);
                }
            }
        }
    }
    None
}

fn autocomplete(data: &Vec<char>) -> Vec<char> {
    let openning_elements = ['<', '(', '{', '['];
    let map: HashMap<char, char> = [('<', '>'), ('(', ')'), ('[', ']'), ('{', '}')].into();

    let mut stack: Vec<char> = Vec::new();

    for c in data {
        if openning_elements.contains(&c) {
            stack.push(*c);
        } else {
            if let Some(stack) = stack.pop() {
                if *c != *map.get(&stack).expect("Incorrect mapping") {
                    return Vec::new();
                }
            }
        }
    }
    stack
        .iter()
        .rev()
        .map(|c| *map.get(&c).expect("Incorrect mapping"))
        .collect()
}

fn part_1(input: &Generated) -> i32 {
    input
        .iter()
        .map(|l| find_first_incorrect_character(&l))
        .filter(|&i| i.is_some())
        .map(|i| match i.unwrap() {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Incorrect closing character"),
        })
        .sum()
}

fn compute_autocomplete_score(data: &Vec<char>) -> u64 {
    data.iter().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Incorrect character"),
            }
    })
}

fn part_2(input: &Generated) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter(|l| find_first_incorrect_character(&l).is_none())
        .map(|l| autocomplete(&l))
        .map(|l| compute_autocomplete_score(&l))
        .collect();

    scores.sort();

    scores[scores.len() / 2]
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
    fn test_find_first_incorrect_character() {
        assert_eq!(
            Some('>'),
            find_first_incorrect_character(&['[', '(', '>', ']'].into())
        );
        assert_eq!(
            None,
            find_first_incorrect_character(&['[', '(', ')', ']'].into())
        );
    }

    #[test]
    fn test_autocomplete() {
        assert_eq!(
            "}}]])})]".chars().collect::<Vec<char>>(),
            autocomplete(&"[({(<(())[]>[[{[]{<()<>>".chars().collect())
        );
    }

    #[test]
    fn test_compute_autocomplete_score() {
        assert_eq!(
            288957,
            compute_autocomplete_score(&"}}]])})]".chars().collect())
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            26397,
            part_1(&generate(
                "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            288957,
            part_2(&generate(
                "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]"
            ))
        );
    }
}
