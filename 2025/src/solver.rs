use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Solver {
    type Ans1: Display;
    type Ans2: Display;

    fn solution1(&self, input: &str) -> Self::Ans1;
    fn solution2(&self, input: &str) -> Self::Ans2;

    fn solve(&self, day: u32, input: &str) {
        let start_1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let a1 = self.solution1(input);
        let end_1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let start_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let a2 = self.solution2(input);
        let end_2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!(">>>>>> ======= Day {:2} ======= <<<<<<", day);
        println!("Answer 1: {}, time elapsed: {:#?}", a1, end_1 - start_1);
        println!("Answer 2: {}, time elapsed: {:#?}", a2, end_2 - start_2);
        println!(">>>>>> ====================== <<<<<<");
    }
}
