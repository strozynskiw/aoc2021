use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Generated = (Vec<u8>, Vec<Vec<u8>>);
struct Image {
    data: HashMap<(isize, isize), u8>,
    top_left_corner: (isize, isize),
    bottom_right_corner: (isize, isize),
}

impl Image {
    fn new() -> Image {
        Image {
            data: HashMap::new(),
            top_left_corner: (0, 0),
            bottom_right_corner: (0, 0),
        }
    }
}

fn generate(input: &str) -> Generated {
    let mut parts = input.split("\n\n");

    let map = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("Incorrect value"),
        })
        .collect();
    let image = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Incorrect value"),
                })
                .collect()
        })
        .collect();
    (map, image)
}

fn get_neighbours(position: &(isize, isize)) -> Vec<(isize, isize)> {
    let mut result: Vec<(isize, isize)> = Vec::new();

    for x in -1..=1 as isize {
        for y in -1..=1 as isize {
            result.push(((position.0 + x) as isize, (position.1 + y) as isize));
        }
    }

    result
}

fn get_value(image: &Image, position: &(isize, isize), void: u8) -> usize {
    let neighbours = get_neighbours(&position);

    let res = neighbours.iter().fold(0, |acc, (x, y)| {
        acc * 2 + *image.data.get(&(*x, *y)).unwrap_or(&void) as usize
    });

    res
}

fn enhance(image: &Image, algorithm: &Vec<u8>, void: u8) -> Image {
    let mut new_image: Image = Image::new();

    new_image.top_left_corner = (image.top_left_corner.0 - 1, image.top_left_corner.1 - 1);
    new_image.bottom_right_corner = (
        image.bottom_right_corner.0 + 1,
        image.bottom_right_corner.1 + 1,
    );

    for x in new_image.top_left_corner.0..new_image.bottom_right_corner.1 {
        for y in new_image.top_left_corner.0..new_image.bottom_right_corner.1 {
            new_image
                .data
                .insert((x, y), algorithm[get_value(image, &(x, y), void)]);
        }
    }

    new_image
}

fn part_1(input: &Generated) -> usize {
    let mut map: HashMap<(isize, isize), u8> = HashMap::new();

    for x in 0..input.1.len() {
        for y in 0..input.1[0].len() {
            map.insert((x as isize, y as isize), input.1[x as usize][y as usize]);
        }
    }

    let mut image = Image {
        data: map,
        top_left_corner: (0, 0),
        bottom_right_corner: (input.1[0].len() as isize, input.1.len() as isize),
    };

    for i in 0..2 {
        let void = if i % 2 == 0 { 0 } else { 1 };
        image = enhance(&image, &input.0, void as u8);
    }

    image.data.values().filter(|v| **v == 1).count()
}

fn part_2(input: &Generated) -> usize {
    let mut map: HashMap<(isize, isize), u8> = HashMap::new();

    for x in 0..input.1.len() {
        for y in 0..input.1[0].len() {
            map.insert((x as isize, y as isize), input.1[x as usize][y as usize]);
        }
    }

    let mut image = Image {
        data: map,
        top_left_corner: (0, 0),
        bottom_right_corner: (input.1[0].len() as isize, input.1.len() as isize),
    };

    for i in 0..50 {
        let void = if i % 2 == 0 { 0 } else { 1 };
        image = enhance(&image, &input.0, void as u8);
    }

    image.data.values().filter(|v| **v == 1).count()
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
