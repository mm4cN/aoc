use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        0
    }

    fn solution2(&self, input: &str) -> Self::Ans2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::*;

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
