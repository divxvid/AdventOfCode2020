use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    ecl: Option<String>,
    hcl: Option<String>,
    pid: Option<String>,
    _cid: Option<String>,
}

impl FromStr for Passport {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut ecl = None;
        let mut hcl = None;
        let mut pid = None;
        let mut cid = None;

        s.split_whitespace()
            .map(|info| info.split_once(":").unwrap())
            .for_each(|(tag, value)| {
                match tag {
                    "byr" => byr = Some(value.to_string()),
                    "iyr" => iyr = Some(value.to_string()),
                    "eyr" => eyr = Some(value.to_string()),
                    "hgt" => hgt = Some(value.to_string()),
                    "ecl" => ecl = Some(value.to_string()),
                    "hcl" => hcl = Some(value.to_string()),
                    "cid" => cid = Some(value.to_string()),
                    "pid" => pid = Some(value.to_string()),
                    _ => panic!("what the ?")
                }
            });
    
        Ok(
            Passport {
                byr,
                iyr,
                eyr,
                hgt,
                ecl,
                hcl,
                pid,
                _cid: cid,
            }
        )
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        check_eyr(self.eyr.clone()) &&
        check_byr(self.byr.clone()) &&
        check_iyr(self.iyr.clone()) &&
        check_hgt(self.hgt.clone()) &&
        check_ecl(self.ecl.clone()) &&
        check_hcl(self.hcl.clone()) &&
        check_pid(self.pid.clone()) 
    }
}

fn check_pid(pid: Option<String>) -> bool {
    if let Some(x) = pid {
        return x.len() == 9
    }
    
    false
}

fn check_hcl(hcl: Option<String>) -> bool {
    if let Some(x) = hcl {
        if x.len() != 7 {
            return false
        }

        return u32::from_str_radix(&x[1..], 16).is_ok();
    }

    false
}

fn check_ecl(ecl: Option<String>) -> bool {
    if let Some(x) = ecl {
        return match x.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false
        }
    }

    false
}

fn check_hgt(hgt: Option<String>) -> bool {
    if let Some(x) = hgt {
        let suffix_start = x.len() - 2;
        let num = x[..suffix_start].parse::<u32>();
        let suffix = &x[suffix_start..];

        if let Ok(x) = num {
            return match suffix {
                "cm" => x >= 150 && x <= 193,
                "in" => x >= 59 && x <= 76,
                _ => false
            }
        }
    }

    false
}

fn validate_year(year: Option<String>, min: u32, max: u32) -> bool {
    if let Some(x) = year {
        if x.len() != 4 {
            return false;
        }

        let num = x.parse::<u32>();
        if let Ok(yr) = num {
            return yr >= min && yr <= max;
        }
    }
    
    false
}

fn check_iyr(iyr: Option<String>) -> bool {
    validate_year(iyr, 2010, 2020)
}

fn check_byr(byr: Option<String>) -> bool {
    validate_year(byr, 1920, 2002)
}

fn check_eyr(eyr: Option<String>) -> bool {
    validate_year(eyr, 2020, 2030)
}

pub fn part_one(input: &str) -> String {
    input.split("\n\n")
        .map(|data| data.parse::<Passport>().unwrap())
        .filter(|passport| {
            let mut present_count = 0;

            present_count += passport.byr.is_some() as u32;
            present_count += passport.iyr.is_some() as u32;
            present_count += passport.eyr.is_some() as u32;
            present_count += passport.hgt.is_some() as u32;
            present_count += passport.ecl.is_some() as u32;
            present_count += passport.hcl.is_some() as u32;
            present_count += passport.pid.is_some() as u32;

            present_count == 7
        })
        .count()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input.split("\n\n")
        .map(|data| data.parse::<Passport>().unwrap())
        .filter(|passport| passport.is_valid())
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let result = part_one(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_two_valid() {
        const INPUT: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let result = part_two(INPUT);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part_two_invalid() {
        const INPUT: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(part_two(INPUT), "0");
    }
}
