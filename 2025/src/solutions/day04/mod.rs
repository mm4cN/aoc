use crate::solver::Solver;
pub struct Problem;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    PaperRoll,
    Forklift,
}
type Grid = Vec<Vec<Field>>;

impl Solver for Problem {
    type Ans1 = u64;
    type Ans2 = u64;

    fn solution1(&self, input: &str) -> Self::Ans1 {
        let grid: Grid = parse_input(input);
        let mut new_grid = grid.clone();

        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        for y in 0..height {
            for x in 0..width {
                if new_grid[y][x] == Field::Empty {
                    continue;
                }
                if let Some(n) = count_adjacent_paper_rolls(&grid, x, y) {
                    if (0..4).contains(&n) {
                        new_grid[y][x] = Field::Forklift;
                    }
                }
            }
        }

        new_grid
            .iter()
            .flatten()
            .filter(|&&f| f == Field::Forklift)
            .count() as u64
    }
    fn solution2(&self, _input: &str) -> Self::Ans2 {
        0
    }
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Field::PaperRoll,
                    '.' => Field::Empty,
                    _ => panic!("Unexpected character in input"),
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_paper_rolls(grid: &Grid, x: usize, y: usize) -> Option<u64> {
    let height = grid.len();
    if y >= height {
        return None;
    }

    let width = grid[y].len();
    if x >= width {
        return None;
    }

    let mut count: u64 = 0;
    for dy in -1isize..=1 {
        for dx in -1isize..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if ny < 0 || ny >= height as isize {
                continue;
            }

            let ny = ny as usize;
            let row_width = grid[ny].len();
            if nx < 0 || nx >= row_width as isize {
                continue;
            }

            let nx = nx as usize;

            if let Field::PaperRoll = grid[ny][nx] {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day04::*;

    #[test]
    fn test_solution1() {
        let problem = Problem {};
        let ans = problem.solution1(TEST_INPUT_1);
        assert_eq!(ans, 13);
    }

    #[test]
    fn test_solution2() {
        let problem = Problem {};
        let ans = problem.solution2(TEST_INPUT_1);
        assert_eq!(ans, 0);
    }

    const TEST_INPUT_1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
}
