use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        }
    }

    fn byr_valid(&self) -> bool {
        return self.byr >= Some(1920) && self.byr <= Some(2002);
    }

    fn ecl_valid(&self) -> bool {
        match &self.ecl {
            Some(ecl) => {
                return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter()
                    .any(|c| {
                        c == ecl
                    })
            },
            None => return false,
        }
    }

    fn eyr_valid(&self) -> bool {
        return self.eyr >= Some(2020) && self.eyr <= Some(2030);
    }

    fn fields_present(&self) -> bool {
        return self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some();
    }

    fn hcl_valid(&self) -> bool {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        match &self.hcl {
            Some(hcl) => return HCL_RE.is_match(&hcl),
            None => return false,
        }
    }

    fn hgt_valid(&self) -> bool {
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)([[:alpha:]]+)$").unwrap();
        }

        match &self.hgt {
            Some(hgt) => {
                match HGT_RE.captures(&hgt) {
                    Some(captures) => {
                        let num: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
                        let unit = captures.get(2).unwrap().as_str();
        
                        match unit {
                            "cm" => return num >= 150 && num <= 193,
                            "in" => return num >= 59 && num <= 76,
                            _ => return false,
                        }
                    },
                    None => return false
                }
            },
            None => return false,
        }
    }

    fn iyr_valid(&self) -> bool {
        return self.iyr >= Some(2010) && self.iyr <= Some(2020);
    }

    fn pid_valid(&self) -> bool {
        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^[[:digit:]]{9}$").unwrap();
        }

        match &self.pid {
            Some(pid) => PID_RE.is_match(&pid),
            None => return false,
        }
    }

    fn valid(&self) -> bool {
        return [
            self.fields_present(),
            self.byr_valid(),
            self.iyr_valid(),
            self.eyr_valid(),
            self.hgt_valid(),
            self.hcl_valid(),
            self.ecl_valid(),
            self.pid_valid()
        ].iter().all(|v| *v)
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    let passports: Vec<_> = input.split("\n\n").map(|e| {
        let fields = split_fields(e);
        parse_passport(&fields)
    }).collect();
    return passports;
}

fn parse_passport(fields: &[&str]) -> Passport {
    let mut passport = Passport::new();

    for field in fields {
        let key = &field[0..3];
        let value = &field[4..];

        match key {
            "byr" => passport.byr = value.parse().ok(),
            "iyr" => passport.iyr = value.parse().ok(),
            "eyr" => passport.eyr = value.parse().ok(),
            "hgt" => passport.hgt = Some(value.to_string()),
            "hcl" => passport.hcl = Some(value.to_string()),
            "ecl" => passport.ecl = Some(value.to_string()),
            "pid" => passport.pid = Some(value.to_string()),
            "cid" => passport.cid = value.parse().ok(),
            _     => panic!("Unknown key: {}", key),
        }
    }

    return passport;
}

fn split_fields(entry: &str) -> Vec<&str> {
    let fields: Vec<&str> = entry.split_whitespace().collect();
    return fields;
}

fn part1(passports: &[Passport]) {
    let num_valid = passports.iter().filter(|p| p.fields_present()).count();
    println!("Valid in part 1: {}", num_valid);
}

fn part2(passports: &[Passport]) {
    let num_valid = passports.iter().filter(|p| p.valid()).count();
    println!("Valid in part 2: {}", num_valid);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let passports = parse_input(&input);

    part1(&passports);
    part2(&passports);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn byr_passport(byr: u16) -> Passport {
        let mut passport = Passport::new();
        passport.byr = Some(byr);
        return passport;
    }

    fn ecl_passport(ecl: String) -> Passport {
        let mut passport = Passport::new();
        passport.ecl = Some(ecl);
        return passport;
    }

    fn eyr_passport(eyr: u16) -> Passport {
        let mut passport = Passport::new();
        passport.eyr = Some(eyr);
        return passport;
    }

    fn hcl_passport(hcl: String) -> Passport {
        let mut passport = Passport::new();
        passport.hcl = Some(hcl);
        return passport;
    }

    fn hgt_passport(hgt: String) -> Passport {
        let mut passport = Passport::new();
        passport.hgt = Some(hgt);
        return passport;
    }

    fn iyr_passport(iyr: u16) -> Passport {
        let mut passport = Passport::new();
        passport.iyr = Some(iyr);
        return passport;
    }

    fn pid_passport(pid: String) -> Passport {
        let mut passport = Passport::new();
        passport.pid = Some(pid);
        return passport;
    }

    #[test]
    fn test_parse_passport() {
        let fields = [
            "ecl:gry",
            "pid:860033327",
            "eyr:2020",
            "hcl:#fffffd",
            "byr:1937",
            "iyr:2017",
            "cid:147",
            "hgt:183cm",
        ];
        let passport = parse_passport(&fields);

        assert_eq!(passport.byr, Some(1937));
        assert_eq!(passport.iyr, Some(2017));
        assert_eq!(passport.eyr, Some(2020));
        assert_eq!(passport.hgt, Some("183cm".to_string()));
        assert_eq!(passport.hcl, Some("#fffffd".to_string()));
        assert_eq!(passport.ecl, Some("gry".to_string()));
        assert_eq!(passport.pid, Some("860033327".to_string()));
        assert_eq!(passport.cid, Some(147));
    }

    #[test]
    fn test_split_fields() {
        let entry = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let fields = split_fields(entry);

        assert_eq!(fields.len(), 8);
        assert_eq!(fields[0], "ecl:gry");
        assert_eq!(fields[1], "pid:860033327");
        assert_eq!(fields[2], "eyr:2020");
        assert_eq!(fields[3], "hcl:#fffffd");
        assert_eq!(fields[4], "byr:1937");
        assert_eq!(fields[5], "iyr:2017");
        assert_eq!(fields[6], "cid:147");
        assert_eq!(fields[7], "hgt:183cm");
    }

    #[test]
    fn test_parse_input() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passports = parse_input(input);
        assert_eq!(passports.len(), 4);

        let passport = &passports[0];
        assert_eq!(passport.byr, Some(1937));
        assert_eq!(passport.iyr, Some(2017));
        assert_eq!(passport.eyr, Some(2020));
        assert_eq!(passport.hgt, Some("183cm".to_string()));
        assert_eq!(passport.hcl, Some("#fffffd".to_string()));
        assert_eq!(passport.ecl, Some("gry".to_string()));
        assert_eq!(passport.pid, Some("860033327".to_string()));
        assert_eq!(passport.cid, Some(147));
    }

    #[test]
    fn test_passport_fields_present_1() {
        let passport = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: Some(147),
        };
        assert!(passport.fields_present());
    }

    #[test]
    fn test_passport_fields_present_2() {
        let passport = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: None,
            cid: Some(147),
        };
        assert!(!passport.fields_present());
    }

    #[test]
    fn test_passport_fields_present_3() {
        let passport = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: None,
        };
        assert!(passport.fields_present());
    }

    #[test]
    fn test_byr_valid_1() {
        let passport = byr_passport(1919);
        assert!(!passport.byr_valid());
    }

    #[test]
    fn test_byr_valid_2() {
        let passport = byr_passport(1980);
        assert!(passport.byr_valid());
    }

    #[test]
    fn test_byr_valid_3() {
        let passport = byr_passport(2003);
        assert!(!passport.byr_valid());
    }

    #[test]
    fn test_byr_valid_4() {
        let passport = Passport::new();
        assert!(!passport.byr_valid());
    }

    #[test]
    fn test_iyr_valid_1() {
        let passport = iyr_passport(2009);
        assert!(!passport.iyr_valid());
    }

    #[test]
    fn test_iyr_valid_2() {
        let passport = iyr_passport(2015);
        assert!(passport.iyr_valid());
    }

    #[test]
    fn test_iyr_valid_3() {
        let passport = iyr_passport(2021);
        assert!(!passport.iyr_valid());
    }

    #[test]
    fn test_iyr_valid_4() {
        let passport = Passport::new();
        assert!(!passport.iyr_valid());
    }

    #[test]
    fn test_eyr_valid_1() {
        let passport = eyr_passport(2019);
        assert!(!passport.eyr_valid());
    }

    #[test]
    fn test_eyr_valid_2() {
        let passport = eyr_passport(2025);
        assert!(passport.eyr_valid());
    }

    #[test]
    fn test_eyr_valid_3() {
        let passport = eyr_passport(2031);
        assert!(!passport.eyr_valid());
    }

    #[test]
    fn test_eyr_valid_4() {
        let passport = Passport::new();
        assert!(!passport.eyr_valid());
    }

    #[test]
    fn test_hgt_passport_1() {
        let passport = hgt_passport("149cm".to_string());
        assert!(!passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_2() {
        let passport = hgt_passport("180cm".to_string());
        assert!(passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_3() {
        let passport = hgt_passport("194cm".to_string());
        assert!(!passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_4() {
        let passport = hgt_passport("58in".to_string());
        assert!(!passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_5() {
        let passport = hgt_passport("65in".to_string());
        assert!(passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_6() {
        let passport = hgt_passport("77in".to_string());
        assert!(!passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_7() {
        let passport = hgt_passport("121sh".to_string());
        assert!(!passport.hgt_valid());
    }

    #[test]
    fn test_hgt_passport_8() {
        let passport = Passport::new();
        assert!(!passport.hgt_valid());
    }

    #[test]
    fn test_hcl_valid_1() {
        let passport = hcl_passport("123456".to_string());
        assert!(!passport.hcl_valid());
    }

    #[test]
    fn test_hcl_valid_2() {
        let passport = hcl_passport("#123abz".to_string());
        assert!(!passport.hcl_valid());
    }

    #[test]
    fn test_hcl_valid_3() {
        let passport = hcl_passport("#123ABC".to_string());
        assert!(!passport.hcl_valid());
    }

    #[test]
    fn test_hcl_valid_4() {
        let passport = hcl_passport("#123abc".to_string());
        assert!(passport.hcl_valid());
    }

    #[test]
    fn test_hcl_valid_5() {
        let passport = hcl_passport("#12345".to_string());
        assert!(!passport.hcl_valid());
    }

    #[test]
    fn test_hcl_valid_6() {
        let passport = Passport::new();
        assert!(!passport.hcl_valid());
    }

    #[test]
    fn test_ecl_valid_1() {
        let passport = ecl_passport("gry".to_string());
        assert!(passport.ecl_valid());
    }

    #[test]
    fn test_ecl_valid_2() {
        let passport = ecl_passport("foo".to_string());
        assert!(!passport.ecl_valid());
    }

    #[test]
    fn test_ecl_valid_3() {
        let passport = Passport::new();
        assert!(!passport.ecl_valid());
    }

    #[test]
    fn test_pid_valid_1() {
        let passport = pid_passport("012345678".to_string());
        assert!(passport.pid_valid());
    }

    #[test]
    fn test_pid_valid_2() {
        let passport = pid_passport("123456".to_string());
        assert!(!passport.pid_valid());
    }

    #[test]
    fn test_pid_valid_3() {
        let passport = pid_passport("#123456789".to_string());
        assert!(!passport.pid_valid());
    }

    #[test]
    fn test_pid_valid_4() {
        let passport = Passport::new();
        assert!(!passport.pid_valid());
    }
}
