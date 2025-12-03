use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        input.lines().map(|line| biggest_in_sequence(line)).sum()
    }

    fn solution2(&self, _input: &str) -> Self::Ans2 {
        0
    }
}

fn biggest_in_sequence(seq: &str) -> u64 {
    let mut max_suffix: Option<u8> = None;
    let mut best: Option<u8> = None;

    for ch in seq.chars().rev() {
        let d = ch.to_digit(10).unwrap() as u8;

        match max_suffix {
            None => {
                max_suffix = Some(d);
            }
            Some(suf) => {
                let value = d * 10 + suf;
                if best.map_or(true, |b| value > b) {
                    best = Some(value);
                }

                if d > suf {
                    max_suffix = Some(d);
                }
            }
        }
    }

    best.map(|v| format!("{:02}", v)).unwrap().parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day03::*;

    #[test]
    fn test_biggest_pair_in_sequence() {
        let seq: &str = "976543211111118";
        let ans: u64 = biggest_in_sequence(seq);
        assert_eq!(ans, 98);
    }

    #[test]
    fn test_solution1() {
        let problem = Problem {};
        let ans = problem.solution1(TEST_INPUT_1);
        assert_eq!(ans, 357);
    }

    #[test]
    fn test_solution2() {
        let problem = Problem {};
        let ans = problem.solution2(TEST_INPUT_1);
        assert_eq!(ans, 0);
    }

    const TEST_INPUT_1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
}
