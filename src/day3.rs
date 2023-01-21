pub fn part_one(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let total_rows = grid.len();
    let total_cols = grid[0].len();

    (0..total_rows).zip((0..).step_by(3))
        .filter(|(r, c)| grid[*r][(*c % total_cols)] == '#')
        .count()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let total_rows = grid.len();
    let total_cols = grid[0].len();

    let strategies = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    strategies.iter()
        .map(|(col_step, row_step)| {
            ((0..total_rows).step_by(*row_step)).zip((0..).step_by(*col_step))
                    .filter(|(r, c)| grid[*r][(*c % total_cols)] == '#')
                    .count()
        })
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "7");
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "336");
    }
}