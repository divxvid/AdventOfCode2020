pub fn part_one(input: &str) -> String {
    input.split("\n\n")
        .map(|group| {
            let mut seen: [bool; 26] = [false; 26];
            for person in group.split_whitespace() {
                for ch in person.chars() {
                    seen[ch as usize - 97] = true;
                }
            }
            seen.iter().filter(|&x| *x).count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input.split("\n\n")
        .map(|group| {
            ('a'..='z')
                .filter(|&ch| {
                    group.split_whitespace().all(|person| {
                        let res = person.contains(ch);
                        res
                    })
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), "11");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), "6");
    }
}