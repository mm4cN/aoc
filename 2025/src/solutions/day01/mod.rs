use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        count_zero_points(input)
    }

    fn solution2(&self, input: &str) -> Self::Ans2 {
        count_zero_crossings(input)
    }
}

struct Rotation {
    dir: i32,
    times: u32,
}
type Rotations = Vec<Rotation>;

fn read_rotations(input: &str) -> Rotations {
    input
        .lines()
        .map(|line| {
            let (dir, times) = line.split_at(1);
            let dir_value = match dir {
                "L" => -1,
                "R" => 1,
                _ => panic!("Unknown direction: {}", dir),
            };
            Rotation {
                dir: dir_value,
                times: times.parse::<u32>().unwrap(),
            }
        })
        .collect()
}

fn count_zero_points(input: &str) -> u64 {
    let rotations = read_rotations(input);
    rotations.iter()
        .scan(50, |curr, rotation| {
            *curr += rotation.dir * (rotation.times as i32);
            *curr %= 100;
            Some(*curr == 0)
        })
        .filter(|&is_zero| is_zero)
        .count() as u64
}

fn count_zero_crossings(input: &str) -> u64 {
    let rotations = read_rotations(input);
    let mut count: u64 = 0;
    let mut curr = 50;
    for rotation in &rotations {
        for _ in 0..rotation.times {
            curr += rotation.dir;
            curr %= 100;
            if curr == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

    #[test]
    fn test_solution1() {
        let problem = Problem{};
        let ans = problem.solution1(TEST_INPUT_1);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_solution2() {
        let problem = Problem{};
        let ans = problem.solution2(TEST_INPUT_1);
        assert_eq!(ans, 6);
    }

    const TEST_INPUT_1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
}
