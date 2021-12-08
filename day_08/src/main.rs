use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Generated<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts = l.split(" | ").collect::<Vec<&str>>();
            (
                parts[0].split_whitespace().collect(),
                parts[1].split_whitespace().collect(),
            )
        })
        .collect()
}

fn part_1(input: &Generated) -> i32 {
    input.iter().fold(0, |acc, v| {
        acc + v
            .1
            .iter()
            .filter(|&i| i.len() == 2 || i.len() == 4 || i.len() == 3 || i.len() == 7)
            .count() as i32
    })
}

fn get_wire_mapping(data: &Vec<&str>) -> HashMap<char, char> {
    let mut map = HashMap::<char, char>::new();

    let one = data.iter().find(|v| v.len() == 2).expect("1 not found");
    let four = data.iter().find(|v| v.len() == 4).expect("4 not found");
    let seven = data.iter().find(|v| v.len() == 3).expect("7 not found");
    let eight = data.iter().find(|v| v.len() == 7).expect("8 not found");

    let segment_a = seven
        .chars()
        .find(|&c| !one.contains(c))
        .expect("A not found");
    map.insert(segment_a, 'a');

    //8-4-7 = eg
    let segment_eg = eight
        .chars()
        .filter(|&c| !four.contains(c) && !seven.contains(c))
        .collect::<Vec<char>>();

    let with_len_5 = data.iter().filter(|v| v.len() == 5).collect::<Vec<&&str>>();
    let two = with_len_5
        .iter()
        .find(|v| v.contains(segment_eg[0]) && v.contains(segment_eg[1]))
        .expect("2 not found");
    let three_five = with_len_5
        .iter()
        .filter(|&item| item != two)
        .collect::<Vec<&&&str>>();

    //7-eg = d
    let segment_d = two
        .chars()
        .find(|&c| !seven.contains(c) && !segment_eg.contains(&c))
        .expect("D not found");
    map.insert(segment_d, 'd');

    //8-7-eg-d = b
    let segment_b = eight
        .chars()
        .find(|&c| !seven.contains(c) && !segment_eg.contains(&c) && c != segment_d)
        .expect("B not found");
    map.insert(segment_b, 'b');

    //4-2-b = f
    let segment_f = four
        .chars()
        .find(|&c| !two.contains(c) && c != segment_b)
        .expect("F not found");
    map.insert(segment_f, 'f');

    //4-a-d-f = c
    let segment_c = four
        .chars()
        .find(|&c| c != segment_b && c != segment_d && c != segment_f)
        .expect("C not found");
    map.insert(segment_c, 'c');

    //2-3/5-c = e
    let segment_e = two
        .chars()
        .find(|&c| !three_five[0].contains(c) && c != segment_c)
        .expect("E not found");
    map.insert(segment_e, 'e');

    //8-a-b-c-d-e-f = g
    let segment_g = eight
        .chars()
        .find(|&c| {
            c != segment_a
                && c != segment_b
                && c != segment_c
                && c != segment_d
                && c != segment_e
                && c != segment_f
        })
        .expect("G not found");
    map.insert(segment_g, 'g');

    map
}

fn part_2(input: &Generated) -> i32 {
    let str_to_digit: HashMap<&str, i32> = [
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]
    .into();

    input
        .iter()
        .map(|l| {
            let map = get_wire_mapping(&l.0);

            let mut mapped =
                l.1.iter()
                    .map(|item| {
                        item.chars()
                            .map(|c| *map.get(&c).expect("Incorrect wire to wire"))
                            .collect::<Vec<char>>()
                    })
                    .collect::<Vec<Vec<char>>>();
            mapped.iter_mut().for_each(|item| item.sort_unstable());
            mapped
                .iter()
                .map(|item| {
                    let s = String::from_iter(item.iter());
                    *str_to_digit
                        .get(s.as_str())
                        .expect("Incorrect str to digit mapping")
                })
                .collect::<Vec<i32>>()
        })
        .map(|item| item[0] * 1000 + item[1] * 100 + item[2] * 10 + item[3])
        .sum()
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
        assert_eq!(26, part_1(&generate("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(61229, part_2(&generate("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")));
    }
}
