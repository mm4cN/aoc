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
            .filter(|&id| repeats_two_times(id))
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
            .filter(|&id| repeats_at_least_two_times(id))
            .sum()
    }
}

fn repeats_two_times(id: u64) -> bool {
    let id_len = match id.to_string().len() >> 1 {
        0 => 1,
        n => n,
    };
    let str_id = id.to_string()[..id_len].to_string();
    match str_id.repeat(2).parse::<u64>() {
        Ok(doubled_id) => doubled_id == id,
        Err(_) => false,
    }
}

fn repeats_at_least_two_times(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for seq_len in 1..=(len / 2) {
        if len % seq_len != 0 {
            continue;
        }
        let reps = len / seq_len;
        if reps < 2 {
            continue;
        }
        let pattern = &s[..seq_len];
        if pattern.repeat(reps) == s {
            return true;
        }
    }
    false
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
