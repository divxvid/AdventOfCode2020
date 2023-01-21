use std::collections::HashSet;

fn parse_boarding_pass(input: &str) -> (u32, u32) {
    let row_info = &input[..7];
    let col_info = &input[7..];

    let mut low = 0;
    let mut high = 127;

    for c in row_info.chars() {
        let mid = (low + high) / 2;
        match c {
            'F' => high = mid,
            'B' => low = mid+1,
            _ => panic!("what the?")
        }
    }

    assert_eq!(low, high);
    let row_number = low;

    low = 0;
    high = 7;

    for c in col_info.chars() {
        let mid = (low + high) / 2;
        match c {
            'L' => high = mid,
            'R' => low = mid+1,
            _ => panic!("what the?")
        }
    }

    assert_eq!(low, high);
    let col_number = low;

    (row_number, col_number)
}

pub fn part_one(input: &str) -> String {
    input.lines()
        .map(|line| parse_boarding_pass(line))
        .map(|(r, c)| r * 8 + c)
        .max().unwrap()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    let seat_ids: HashSet<u32> = input.lines()
        .map(|line| parse_boarding_pass(line))
        .map(|(r, c)| r * 8 + c)
        .collect();

    let last_id = 127 * 8 + 7;

    (1..last_id)
        .find(|i| seat_ids.contains(&(i-1)) &&
                seat_ids.contains(&(i+1)) &&
                !seat_ids.contains(&i)
        ).unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_boarding_pass("BFFFBBFRRR"), (70, 7));
        assert_eq!(parse_boarding_pass("FFFBBBFRRR"), (14, 7));
        assert_eq!(parse_boarding_pass("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

        assert_eq!(part_one(INPUT), "820");
    }
}