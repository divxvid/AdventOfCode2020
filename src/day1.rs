use std::collections::{HashSet, HashMap};

pub fn part_one(input: &str) -> String {
    let data: Vec<u32> = input.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let mut seen: HashSet<u32> = HashSet::new(); 
    
    for i in 0..data.len() {
        let need = 2020 - data[i];
        if seen.contains(&need) {
            return (need * data[i]).to_string();
        }

        seen.insert(data[i]);
    }

    return "Nope".to_string();
}

pub fn part_two(input: &str) -> String {
    let data: Vec<i64> = input.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut seen: HashMap<i64, usize> = HashMap::new();
    
    for i in 0..data.len() {
        for j in 0..i {
            let need = 2020 - (data[i] + data[j]);
            if seen.contains_key(&need) {
                let index = seen.get(&need).unwrap();
                if *index != j {
                    let result = data[i] * data[j] * need;
                    return result.to_string();
                }
            }
        }
        seen.insert(data[i], i);
    }

    return "Nope".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "514579");
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "241861950");
    }
}