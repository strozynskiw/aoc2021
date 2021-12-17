use std::ops::RangeInclusive;
use std::time::Instant;

type Generated = Target;

struct Target {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

fn generate() -> Generated {
    Target {
        x: 169..=206,
        y: -108..=-68,
    }
}

fn make_step(run: &mut Run, target: &Target) -> bool {
    run.make_step();

    target.x.contains(&run.position.0) && target.y.contains(&run.position.1)
}

#[derive(Debug)]
struct Run {
    vel_x: i64,
    vel_y: i64,
    position: (i64, i64),
}

impl Run {
    fn make_step(&mut self) {
        self.position.0 += self.vel_x;
        self.position.1 += self.vel_y;

        if self.vel_x > 0 {
            self.vel_x -= 1;
        }
        if self.vel_x < 0 {
            self.vel_x += 1;
        }
        self.vel_y -= 1;
    }
}

#[derive(Debug)]
struct SimuResult {
    hit: bool,
    high: i64,
}

fn simulation(vel_x: i64, vel_y: i64, target: &Target) -> SimuResult {
    let mut run = Run {
        position: (0, 0),
        vel_x,
        vel_y,
    };

    let mut high = i64::MIN;

    loop {
        if run.position.0 > *target.x.end() || run.position.1 < *target.y.start() {
            return SimuResult { hit: false, high };
        }

        let hit = make_step(&mut run, target);

        high = high.max(run.position.1);

        if hit {
            return SimuResult { hit: true, high };
        }


    }
}

fn part_1(input: &Generated) -> i64 {
    let mut high = i64::MIN;
    for vel_x in 0..300 {
        for vel_y in -1000..1000 {
            let res = simulation(vel_x, vel_y, input);
            if res.hit {
                high = high.max(res.high);
            }
        }
    }

    high
}

fn part_2(input: &Generated) -> i64 {
    let mut hits = 0;
    for vel_x in -10..300 {
        for vel_y in -1000..10000 {
            let res = simulation(vel_x, vel_y, input);
            if res.hit {
                hits += 1;
            }
        }
    }
    hits
}

fn main() {
    // let content = fs::read_to_string("input").expect("file not found");

    let data = generate();

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
            45,
            part_1(&Target {
                x: 20..=30,
                y: -10..=-5
            })
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            112,
            part_2(&Target {
                x: 20..=30,
                y: -10..=-5
            })
        );
    }
}
