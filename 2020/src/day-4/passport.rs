use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BYR_RE: Regex =
        Regex::new(r"(^|\s+)byr:(?P<byr>\d{4})($|\s+)").expect("regex is valid");
    static ref IYR_RE: Regex =
        Regex::new(r"(^|\s+)iyr:(?P<iyr>\d{4})($|\s+)").expect("regex is valid");
    static ref EYR_RE: Regex =
        Regex::new(r"(^|\s+)eyr:(?P<eyr>\d{4})($|\s+)").expect("regex is valid");
    static ref HGT_RE: Regex =
        Regex::new(r"(^|\s+)hgt:(?P<hgt>\d{3}cm|\d{2}in)($|\s+)").expect("regex is valid");
    static ref HCL_RE: Regex =
        Regex::new(r"(^|\s+)hcl:#(?P<hcl>[0-9a-f]{6})($|\s+)").expect("regex is valid");
    static ref ECL_RE: Regex =
        Regex::new(r"(^|\s+)ecl:(?P<ecl>amb|blu|brn|gry|grn|hzl|oth)($|\s+)")
            .expect("regex is valid");
    static ref PID_RE: Regex =
        Regex::new(r"(^|\s+)pid:(?P<pid>\d{9})($|\s+)").expect("regex is valid");
}

#[derive(Debug)]
pub struct Passport {
    byr: String, // birth year
    iyr: String, // issue year
    eyr: String, // expiration year
    hgt: String, // height
    hcl: String, // hair colour
    ecl: String, // eye colour
    pid: String, // passport id

    has_byr: bool, // birth year
    has_iyr: bool, // issue year
    has_eyr: bool, // expiration year
    has_hgt: bool, // height
    has_hcl: bool, // hair colour
    has_ecl: bool, // eye colour
    has_pid: bool, // passport id
                   // cid: String, // country id, will leave commented since we skip it in day-4
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            byr: String::from(""),
            iyr: String::from(""),
            eyr: String::from(""),
            hgt: String::from(""),
            hcl: String::from(""),
            ecl: String::from(""),
            pid: String::from(""),

            has_byr: false,
            has_iyr: false,
            has_eyr: false,
            has_hgt: false,
            has_hcl: false,
            has_ecl: false,
            has_pid: false,
        }
    }

    pub fn add_input_data(&mut self, input: &str) {
        self.add_byr(input);
        self.add_iyr(input);
        self.add_eyr(input);
        self.add_hgt(input);
        self.add_hcl(input);
        self.add_ecl(input);
        self.add_pid(input);
    }

    fn add_byr(&mut self, input: &str) {
        if !input.contains("byr") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_byr = true;

        let capture = BYR_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("byr");
            if let Some(val) = named_match {
                self.byr = val.as_str().to_string();
            }
        }
    }

    fn add_iyr(&mut self, input: &str) {
        if !input.contains("iyr") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_iyr = true;

        let capture = IYR_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("iyr");
            if let Some(val) = named_match {
                self.iyr = val.as_str().to_string();
            }
        }
    }

    fn add_eyr(&mut self, input: &str) {
        if !input.contains("eyr") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_eyr = true;

        let capture = EYR_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("eyr");
            if let Some(val) = named_match {
                self.eyr = val.as_str().to_string();
            }
        }
    }

    fn add_hgt(&mut self, input: &str) {
        if !input.contains("hgt") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_hgt = true;

        let capture = HGT_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("hgt");
            if let Some(val) = named_match {
                self.hgt = val.as_str().to_string();
            }
        }
    }

    fn add_hcl(&mut self, input: &str) {
        if !input.contains("hcl") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_hcl = true;

        let capture = HCL_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("hcl");
            if let Some(val) = named_match {
                self.hcl = val.as_str().to_string();
            }
        }
    }

    fn add_ecl(&mut self, input: &str) {
        if !input.contains("ecl") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_ecl = true;

        let capture = ECL_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("ecl");
            if let Some(val) = named_match {
                self.ecl = val.as_str().to_string();
            }
        }
    }

    fn add_pid(&mut self, input: &str) {
        if !input.contains("pid") {
            return;
        }

        // mark it regardless of validation (part 1)
        self.has_pid = true;

        let capture = PID_RE.captures(input);

        if let Some(re_match) = capture {
            let named_match = re_match.name("pid");
            if let Some(val) = named_match {
                self.pid = val.as_str().to_string();
            }
        }
    }

    pub fn is_valid(&self, check_values: bool) -> bool {
        if check_values {
            self.has_valid_values()
        } else {
            self.has_all_values()
        }
    }

    // check has_val bools (part 1)
    fn has_all_values(&self) -> bool {
        self.has_byr
            && self.has_iyr
            && self.has_eyr
            && self.has_hgt
            && self.has_hcl
            && self.has_ecl
            && self.has_pid
    }

    // check if val member is actually set to non-empty string (part 2)
    fn has_all_values_set(&self) -> bool {
        !self.byr.is_empty()
            && !self.iyr.is_empty()
            && !self.eyr.is_empty()
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }

    fn has_valid_values(&self) -> bool {
        if !self.has_all_values_set() {
            return false;
        }

        // byr is 1920-2002
        // we can unwrap safely since the input is validated by the regex
        let byr = self.byr.parse::<u16>().unwrap();
        if !(1920..=2002).contains(&byr) {
            return false;
        }

        // iyr is between 2010-2020
        let iyr = self.iyr.parse::<u16>().unwrap();
        if !(2010..=2020).contains(&iyr) {
            return false;
        }

        // eyr is between 2020-2030
        let eyr = self.eyr.parse::<u16>().unwrap();
        if !(2020..=2030).contains(&eyr) {
            return false;
        }

        // hgt
        if self.hgt.contains("cm") {
            let height = self.hgt[0..=2].parse::<u16>().unwrap();
            if !(150..=193).contains(&height) {
                return false;
            }
        } else if self.hgt.contains("in") {
            let height = self.hgt[0..=1].parse::<u8>().unwrap();
            if !(59..=76).contains(&height) {
                return false;
            }
        }

        true
    }
}
