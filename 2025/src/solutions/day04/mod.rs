use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, _input: &str) -> Self::Ans1 {
        0
    }

    fn solution2(&self, _input: &str) -> Self::Ans2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day04::*;

    #[test]
    fn test_solution1() {
        let problem = Problem {};
        let ans = problem.solution1(TEST_INPUT_1);
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_solution2() {
        let problem = Problem {};
        let ans = problem.solution2(TEST_INPUT_1);
        assert_eq!(ans, 0);
    }

    const TEST_INPUT_1: &str = "";
}
