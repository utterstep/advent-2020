use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct Passport<'a>(BTreeMap<&'a str, &'a str>);

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl<'a> Passport<'a> {
    pub(crate) fn new(raw_data: &'a str) -> Self {
        let mut map = BTreeMap::new();

        raw_data.split(|c| c == ' ' || c == '\n').for_each(|field| {
            let mut data = field.split(':');

            if let Some(key) = data.next() {
                if let Some(value) = data.next() {
                    map.insert(key.trim(), value.trim());
                }
            }
        });

        Self(map)
    }

    pub(crate) fn contains_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|&field| self.0.contains_key(field))
    }

    pub(crate) fn is_valid(&self) -> bool {
        self._is_valid().unwrap_or(false)
    }

    fn _is_valid(&self) -> Option<bool> {
        let byr = self.0.get("byr")?.parse::<u32>().ok()?;

        if !(1920..=2002).contains(&byr) {
            return Some(false);
        }

        let iyr = self.0.get("iyr")?.parse::<u32>().ok()?;

        if !(2010..=2020).contains(&iyr) {
            return Some(false);
        }

        let eyr = self.0.get("eyr")?.parse::<u32>().ok()?;

        if !(2010..=2030).contains(&eyr) {
            return Some(false);
        }

        let hgt = self.0.get("hgt")?;

        if let Some(hgt) = hgt.strip_suffix("cm") {
            let hgt = hgt.parse::<u32>().ok()?;

            if !(150..=193).contains(&hgt) {
                return Some(false);
            }
        } else if let Some(hgt) = hgt.strip_suffix("in") {
            let hgt = hgt.parse::<u32>().ok()?;

            if !(59..=76).contains(&hgt) {
                return Some(false);
            }
        } else {
            return Some(false);
        }

        let hcl = self.0.get("hcl")?;
        let hcl = hcl.strip_prefix('#')?;

        if hcl.len() != 6 {
            return Some(false);
        }

        u32::from_str_radix(hcl, 16).ok()?;

        let ecl = self.0.get("ecl")?;

        const EYE_COLLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        if !EYE_COLLORS.contains(ecl) {
            return Some(false);
        }

        let pid = self.0.get("pid")?;

        if pid.len() != 9 {
            return Some(false);
        }

        pid.parse::<u32>().ok()?;

        Some(true)
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
                .map(Passport::new)
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
            .map(Passport::new)
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
            .map(Passport::new)
            .all(|p| p.is_valid()));
    }
}
