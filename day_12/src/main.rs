use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Generated<'a> = HashMap<&'a str, Vec<&'a str>>;

fn generate(input: &str) -> Generated {
    let mut map: Generated = HashMap::new();
    let pairs = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split('-').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for pair in pairs {
        map.entry(pair[0]).or_default().push(pair[1]);
        map.entry(pair[1]).or_default().push(pair[0]);
    }

    map
}

fn ascii_is_capitalized(name: &str) -> bool {
    name.chars().all(|c| c as u8 > 64 && c as u8 <= 90)
}

fn can_follow(visited: &Vec<&str>, destination: &str) -> bool {
    ascii_is_capitalized(destination) || !visited.contains(&destination)
}

fn only_one_appears_twice(visited: &Vec<&str>) -> bool {
    let only_small = visited
        .iter()
        .filter(|v| !ascii_is_capitalized(v))
        .collect::<Vec<&&str>>();

    let mut duplicates = 0;
    for (i, n1) in only_small.iter().enumerate() {
        for (j, n2) in only_small.iter().enumerate() {
            if i != j && n1 == n2 {
                duplicates += 1;
            }
        }
    }

    duplicates <= 2
}

fn can_follow2(visited: &Vec<&str>, destination: &str) -> bool {
    let mut path = visited.clone();
    path.push(destination);
    ascii_is_capitalized(destination) || (destination != "start" && only_one_appears_twice(&path))
}

fn go(
    visited: &Vec<&str>,
    node: &str,
    map: &Generated,
    can_follow: &dyn Fn(&Vec<&str>, &str) -> bool,
) -> i32 {
    if node == "end" {
        return 1;
    }

    let mut paths = 0;

    let edges = map.get(node).unwrap();
    let mut new_visited = visited.clone();
    new_visited.push(node);
    for edge in edges {
        if can_follow(&new_visited, edge) {
            paths += go(&new_visited.clone(), edge, map, &can_follow)
        }
    }

    paths
}

fn part_1(input: &Generated) -> i32 {
    let visited: Vec<&str> = Vec::new();
    go(&visited, "start", input, &can_follow)
}

fn part_2(input: &Generated) -> i32 {
    let visited: Vec<&str> = Vec::new();
    go(&visited, "start", input, &can_follow2)
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let data = generate(&content);

    let res1_start = Instant::now();
    let res1 = part_1(&data);
    let res1_stop = Instant::now();
    print!(
        "Result1: {}\nResolved in: {:?}\n",
        res1,
        res1_stop.duration_since(res1_start)
    );

    let res2_start = Instant::now();
    let res2 = part_2(&data);
    let res2_stop = Instant::now();
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
    fn test_only_one_appears_twice() {
        assert_eq!(true, only_one_appears_twice(&["aa", "bb", "aa"].to_vec()));
        assert_eq!(true, only_one_appears_twice(&["aa", "aa"].to_vec()));
        assert_eq!(
            false,
            only_one_appears_twice(&["aa", "bb", "aa", "bb"].to_vec())
        );
        assert_eq!(true, only_one_appears_twice(&["aa", "bb", "cc"].to_vec()));
        assert_eq!(
            true,
            only_one_appears_twice(&["AA", "AA", "AA", "AA", "aa", "bb", "cc"].to_vec())
        );
        assert_eq!(true, only_one_appears_twice(&[].to_vec()));
        assert_eq!(true, only_one_appears_twice(&["aa"].to_vec()));
    }

    #[test]
    fn test_can_follow2() {
        assert_eq!(true, can_follow2(&["aa", "bb", "aa"].to_vec(), "xx"));
        assert_eq!(false, can_follow2(&["aa", "aa"].to_vec(), "aa"));
        assert_eq!(false, can_follow2(&["aa", "bb", "aa", "bb"].to_vec(), "xx"));
        assert_eq!(true, can_follow2(&["aa", "bb", "cc"].to_vec(), "bb"));
        assert_eq!(
            true,
            can_follow2(&["AA", "AA", "AA", "AA", "aa", "bb", "cc"].to_vec(), "AA")
        );
        assert_eq!(true, can_follow2(&[].to_vec(), "xx"));
        assert_eq!(false, can_follow2(&["start"].to_vec(), "start"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            226,
            part_1(&generate(
                "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            ))
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            36,
            part_2(&generate(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ))
        );
    }
}
