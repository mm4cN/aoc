use crate::solver::Solver;
pub struct Problem;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, _input: &str) -> Self::Ans1 {
        let (ranges, ids) = parse_input(_input);
        ids.iter()
            .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
            .count() as u64
    }

    fn solution2(&self, _input: &str) -> Self::Ans2 {
        let (mut ranges, _) = parse_input(_input);
        ranges.sort_by_key(|&(start, _)| start);

        let mut merged: Vec<(u64, u64)> = Vec::new();
        for &(start, end) in &ranges {
            if let Some(&mut (ref mut _m_start, ref mut m_end)) = merged.last_mut() {
                if start <= *m_end + 1 {
                    *m_end = (*m_end).max(end);
                } else {
                    merged.push((start, end));
                }
            } else {
                merged.push((start, end));
            }
        }

        merged
            .iter()
            .map(|&(start, end)| end - start + 1)
            .sum::<u64>()
    }
}

type Range = (u64, u64);
type Id = u64;

fn parse_input(input: &str) -> (Vec<Range>, Vec<Id>) {
    let mut sections = input.split("\n\n");
    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();

    let ranges: Vec<Range> = ranges_section
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let ids: Vec<Id> = ids_section
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day05::*;

    #[test]
    fn test_solution1() {
        let problem = Problem {};
        let ans = problem.solution1(TEST_INPUT_1);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_solution2() {
        let problem = Problem {};
        let ans = problem.solution2(TEST_INPUT_1);
        assert_eq!(ans, 14);
    }

    const TEST_INPUT_1: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
}
