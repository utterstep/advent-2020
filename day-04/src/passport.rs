use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug)]
pub(crate) struct Passport(BTreeMap<String, String>);

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::new();

        s.split(|c| c == ' ' || c == '\n').for_each(|field| {
            let mut data = field.split(':');

            if let Some(key) = data.next() {
                if let Some(value) = data.next() {
                    map.insert(key.trim().to_owned(), value.trim().to_owned());
                }
            }
        });

        Ok(Self(map))
    }
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    pub(crate) fn contains_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|&field| self.0.contains_key(field))
    }

    pub(crate) fn is_valid(&self) -> bool {
        if !self.contains_required_fields() {
            return false;
        }

        let byr = self.0.get("byr").unwrap().parse::<u32>();

        if let Ok(byr) = byr {
            if !(byr >= 1920 && byr <= 2002) {
                return false;
            }
        } else {
            return false;
        }

        let iyr = self.0.get("iyr").unwrap().parse::<u32>();

        if let Ok(iyr) = iyr {
            if !(iyr >= 2010 && iyr <= 2020) {
                return false;
            }
        } else {
            return false;
        }

        let eyr = self.0.get("eyr").unwrap().parse::<u32>();

        if let Ok(eyr) = eyr {
            if !(eyr >= 2010 && eyr <= 2030) {
                return false;
            }
        } else {
            return false;
        }

        let hgt = self.0.get("hgt").unwrap();

        if hgt.ends_with("cm") {
            if let Ok(hgt) = hgt.split_at(hgt.len() - 2).0.parse::<u32>() {
                if !(hgt >= 150 && hgt <= 193) {
                    return false;
                }
            }
        } else if hgt.ends_with("in") {
            if let Ok(hgt) = hgt.split_at(hgt.len() - 2).0.parse::<u32>() {
                if !(hgt >= 59 && hgt <= 76) {
                    return false;
                }
            }
        } else {
            return false;
        }

        let hcl = self.0.get("hcl").unwrap();

        if !hcl.starts_with('#') {
            return false;
        }

        if hcl[1..].len() != 6 {
            return false;
        }

        if u32::from_str_radix(hcl[1..].as_ref(), 16).is_err() {
            return false;
        }

        let ecl = self.0.get("ecl").unwrap();

        const EYE_COLLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        if !EYE_COLLORS.contains(&ecl.as_ref()) {
            return false;
        }

        let pid = self.0.get("pid").unwrap();

        if pid.len() != 9 {
            return false;
        }

        if pid.parse::<u32>().is_err() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let passports = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        assert_eq!(
            passports
                .split("\n\n")
                .map(str::parse::<Passport>)
                .map(Result::unwrap)
                .map(|p| p.contains_required_fields())
                .collect::<Vec<_>>(),
            vec![true, false, true, false],
        )
    }

    #[test]
    fn test_part_two_examples() {
        let invalid_passports = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

        assert!(invalid_passports
            .split("\n\n")
            .map(str::parse::<Passport>)
            .map(Result::unwrap)
            .all(|p| !p.is_valid()));

        let valid_passports = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert!(valid_passports
            .split("\n\n")
            .map(str::parse::<Passport>)
            .map(Result::unwrap)
            .all(|p| p.is_valid()));
    }
}
