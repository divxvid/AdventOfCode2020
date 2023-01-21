use nom::{bytes::streaming::tag, IResult};

#[derive(Debug)]
struct InputLine {
    lower_range: u32,
    upper_range: u32,
    character: char,
    password: String
}

fn parse_line(s: &str) -> IResult<&str, InputLine> {
    let (s, lower_range) = nom::character::complete::u32(s)?;
    let (s, _) = tag("-")(s)?;
    let (s, upper_range) = nom::character::complete::u32(s)?;
    let (s, _) = tag(" ")(s)?;
    let (s, character) = nom::character::complete::alpha1(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, password) = nom::character::complete::alpha1(s)?;

    let character = character.chars().next().unwrap();
    let password = password.to_string();
    
    Ok(
        (s, InputLine{
            lower_range,
            upper_range,
            character,
            password,
        })
    )
}

pub fn part_one(input: &str) -> String {
    input.lines()
        .map(|line| parse_line(line).unwrap().1)
        .filter(|x| {
            let count = x.password.chars()
                .filter(|ch| *ch == x.character)
                .count() as u32;
            
            return (count >= x.lower_range) && (count <= x.upper_range);
        })
        .count()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input.lines()
        .map(|line| parse_line(line).unwrap().1)
        .filter(|x| {
            let c1 = x.password.chars().nth((x.lower_range-1) as usize).unwrap();
            let c2 = x.password.chars().nth((x.upper_range-1) as usize).unwrap();

            if c1 == x.character && c2 == x.character {
                return false;
            }

            if c1 == x.character || c2 == x.character {
                return true;
            }

            return false;
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "2");
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "1");
    }
}