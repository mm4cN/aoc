use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        input
            .trim()
            .split(',')
            .map(|part| {
                let mut bounds = part.split('-').map(|x| x.parse::<u64>().unwrap());
                let start = bounds.next().unwrap();
                let end = bounds.next().unwrap();
                (start, end)
            })
            .flat_map(|(start, end)| start..=end)
            .filter(|&id| repeats(id, true))
            .sum()
    }

    fn solution2(&self, input: &str) -> Self::Ans2 {
        input
            .trim()
            .split(',')
            .map(|part| {
                let mut bounds = part.split('-').map(|x| x.parse::<u64>().unwrap());
                let start = bounds.next().unwrap();
                let end = bounds.next().unwrap();
                (start, end)
            })
            .flat_map(|(start, end)| start..=end)
            .filter(|&id| repeats(id, false))
            .sum()
    }
}

fn repeats(id: u64, exact: bool) -> bool {
    let s = id.to_string();
    let len = s.len();
    let s_bytes = s.as_bytes();

    (1..=(len / 2)).any(|seq_len| {
        len % seq_len == 0 &&
        {
            let reps = len / seq_len;
            (!exact && reps >= 2 || exact && reps == 2) &&
            {
                let pattern = &s_bytes[..seq_len];
                s_bytes.chunks(seq_len).all(|chunk| chunk == pattern)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::solutions::day02::*;

    #[test]
    fn test_solution1() {
        let problem = Problem {};
        let ans = problem.solution1(TEST_INPUT_1);
        assert_eq!(ans, 1227775554);
    }

    #[test]
    fn test_solution2() {
        let problem = Problem {};
        let ans = problem.solution2(TEST_INPUT_1);
        assert_eq!(ans, 4174379265);
    }

    const TEST_INPUT_1: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";
}
