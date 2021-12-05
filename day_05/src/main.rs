use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

type Generated = Vec<Translation>;

#[derive(Debug)]
struct Translation {
    from: (i32, i32),
    to: (i32, i32),
}

impl FromStr for Translation {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" -> ").collect::<Vec<&str>>();
        let values = parts
            .iter()
            .map(|p| {
                p.split(',')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Ok(Translation {
            from: (values[0][0], values[0][1]),
            to: (values[1][0], values[1][1]),
        })
    }
}

struct TranslationWalker<'a> {
    translation: &'a Translation,
    position: (i32, i32),
    step: (i32, i32),
    exhausted: bool,
}

impl TranslationWalker<'_> {
    fn from_translation(translation: &Translation) -> TranslationWalker {
        TranslationWalker {
            translation,
            position: translation.from,
            step: (
                i32::signum(translation.to.0 - translation.from.0),
                i32::signum(translation.to.1 - translation.from.1),
            ),
            exhausted: false,
        }
    }
}

impl Iterator for TranslationWalker<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else if self.position == self.translation.to {
            self.exhausted = true;
            Some(self.position)
        } else {
            let res = Some(self.position);

            self.position.0 += self.step.0;
            self.position.1 += self.step.1;

            res
        }
    }
}

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Translation::from_str(l).unwrap())
        .collect()
}

fn part_1(input: &Generated) -> usize {
    let only_strait: Vec<&Translation> = input
        .iter()
        .filter(|i| i.from.0 == i.to.0 || i.from.1 == i.to.1)
        .collect();

    let mut map: HashMap<(i32, i32), usize> = HashMap::new();

    only_strait
        .iter()
        .map(|t| TranslationWalker::from_translation(t))
        .for_each(|walker| {
            walker.into_iter().for_each(|step| {
                *map.entry(step).or_insert(0) += 1
            })
        });

    map.iter().filter(|(_, &v)| v > 1).count()
}

fn part_2(input: &Generated) -> usize {
    let mut map: HashMap<(i32, i32), usize> = HashMap::new();

    input
        .iter()
        .map(TranslationWalker::from_translation)
        .for_each(|walker| {
            walker.into_iter().for_each(|step| {
                let val = map.entry(step).or_insert(0);
                *val += 1
            })
        });

    map.iter().filter(|(_, &v)| v > 1).count()
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
    fn test_iter_for_translation() {
        let mut iter = TranslationWalker::from_translation(&Translation {
            from: (0, 0),
            to: (2, 2),
        })
        .into_iter();

        assert_eq!(Some((0, 0)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(Some((2, 2)), iter.next());
        assert_eq!(None, iter.next());

        let mut iter = TranslationWalker::from_translation(&Translation {
            from: (2, 2),
            to: (0, 0),
        })
        .into_iter();

        assert_eq!(Some((2, 2)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(Some((0, 0)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            5,
            part_1(&generate(
                r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            ))
        )
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            12,
            part_2(&generate(
                r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            ))
        )
    }
}
