use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        input.lines().map(|line| biggest_k_in_sequence(line, 2)).sum()
    }

    fn solution2(&self, input: &str) -> Self::Ans2 {
        input.lines().map(|line| biggest_k_in_sequence(line, 12)).sum()
    }
}

fn biggest_k_in_sequence(seq: &str, k: usize) -> u64 {
    let digits: Vec<u8> = seq.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut to_remove = digits.len().saturating_sub(k);

    for &d in &digits {
        while let Some(&last) = stack.last() {
            if to_remove > 0 && last < d {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(d);
    }

    stack.truncate(k);

    let s: String = stack.iter().map(|d| (b'0' + *d) as char).collect();
    s.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day03::*;

    #[test]
    fn test_biggest_k_in_sequence() {
        let seq: &str = "976543211111118";
        let ans: u64 = biggest_k_in_sequence(seq, 2);
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
        assert_eq!(ans, 3121910778619);
    }

    const TEST_INPUT_1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
}
