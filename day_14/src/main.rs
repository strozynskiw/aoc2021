use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Generated<'a> = (&'a str, HashMap<Vec<char>, char>);

fn generate(input: &str) -> Generated {
    let mut parts = input.split("\n\n");
    let molecule = parts.next().unwrap();
    let rules = parts.next().unwrap();

    let rules = rules
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(" -> ");
            (
                parts.next().unwrap().chars().take(2).collect::<Vec<char>>(),
                parts.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect::<HashMap<Vec<char>, char>>();

    (molecule, rules)
}

fn initialize(
    data: &Generated,
    collection: &mut HashMap<char, usize>,
    subproducts: &mut HashMap<Vec<char>, usize>,
) {
    // extract pairs and count initial number of elements
    data.0
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .for_each(|w| {
            *subproducts.entry(w.to_vec()).or_default() += 1;
            *collection.entry(w[0]).or_default() += 1
        });

    // don't forget to the last character - not included above
    *collection
        .entry(data.0.chars().last().unwrap())
        .or_default() += 1;
}

fn synthetize_step(
    collection: &mut HashMap<char, usize>,
    subproducts: &HashMap<Vec<char>, usize>,
    rules: &HashMap<Vec<char>, char>,
) -> HashMap<Vec<char>, usize> {
    let mut new_subproducts: HashMap<Vec<char>, usize> = HashMap::new();

    for sub in subproducts {
        if let Some(c) = rules.get(sub.0) {
            *new_subproducts.entry([sub.0[0], *c].to_vec()).or_default() += sub.1;
            *new_subproducts.entry([*c, sub.0[1]].to_vec()).or_default() += sub.1;

            *collection.entry(*c).or_default() += sub.1;
        }
    }

    new_subproducts
}

fn synthetize(data: &Generated, iterations: usize) -> usize {
    let mut collection: HashMap<char, usize> = HashMap::new();
    let mut subproducts: HashMap<Vec<char>, usize> = HashMap::new();

    initialize(data, &mut collection, &mut subproducts);

    for _ in 0..iterations {
        subproducts = synthetize_step(&mut collection, &subproducts, &data.1);
    }

    let max = collection.values().max().unwrap();
    let min = collection.values().min().unwrap();

    max - min
}

fn part_1(input: &Generated) -> usize {
    synthetize(input, 10)
}

fn part_2(input: &Generated) -> usize {
    synthetize(input, 40)
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
            1588,
            part_1(&generate(
                "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            2188189693529,
            part_2(&generate(
                "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C"
            ))
        );
    }
}
