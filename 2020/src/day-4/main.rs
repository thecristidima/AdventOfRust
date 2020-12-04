use utils::files::read_lines_iter;

mod passport;

use passport::Passport;

/**
 * You noticed you brought your North Pole Credentials instead of passport.
 * Many people seem to have the same problem as you, so you decide to hack
 *  into the airport terminals and change the checks.
 *
 * Each passport has the following data:
 * 	- byr (birth year)
 *  - iyr (issue year)
 *  - eyr (expiration year)
 *  - hgt (height)
 *  - hcl (hair colour)
 *  - ecl (eye colour)
 *  - pid (passport id)
 *  - cid (country id)
 *
 * Your North Pole Credentials contains everything, but the cid, so you
 *  decide to hack the computers to skip that field. Any other missing
 *  field will still count as an invalid passport.
 *
 * The input contains the passport fields on an arbitrary number of lines.
 * A line break indicates there are no more fields to process for the
 *  current passport.
 *
 * Part 1:
 *  How many passports are valid using the updated check?
 *
 * Part 2:
 *  The security guards noticed that some invalid fields are getting
 *   through the checks. You decide to validate the data you get
 *   (you still ignore cid).
 *
 *  These are the rules:
 *   - byr: 4 digits, between 1920-2002
 *   - iyr: 4 digits, between 2010-2020
 *   - eyr: 4 digits, between 2020-2030
 *   - hgt: number followed by eiher cm or in
 *     - if cm, 3 digits, between 150-193
 *     - if in, 2 digits, between 59-76
 *   - hcl: a # followed by 6 characters 0-f
 *   - ecl: one of amb, blu, brn, gry, grn, hzl, oth
 *   - pid: 9 digits including leading zeroes
 *
 */
fn main() {
    let mut crt_passport = Passport::new();
    let mut passports_with_all_fields: u32 = 0; // part 1
    let mut passports_with_all_valid_fields: u32 = 0; // part 2

    for line in read_lines_iter("src/day-4/input.txt") {
        let line = line.expect("input is valid line");

        if line.is_empty() {
            if crt_passport.is_valid(false) {
                passports_with_all_fields += 1;
            }

            if crt_passport.is_valid(true) {
                passports_with_all_valid_fields += 1;
            }

            crt_passport = Passport::new();
        } else {
            crt_passport.add_input_data(&line);
        }
    }

    println!("Solution for part 1: {}", passports_with_all_fields); // 245
    println!("Solution for part 2: {}", passports_with_all_valid_fields); // 133
}
