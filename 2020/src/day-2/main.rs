use utils::files::read_lines_iter;

// Returns min, max, char to check and password
fn split_line(line: String) -> (i32, i32, char, String) {
    let mut iter = line.split_ascii_whitespace();

    let range = iter.next().expect("line starts with range...");

    let min = range
        .split('-')
        .nth(0)
        .unwrap()
        .parse::<i32>()
        .expect("first part is a number");

    let max = range
        .split('-')
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .expect("second part is also a number");

    let char_to_check = iter
        .next()
        .expect("... continues with `$char:` ...")
        .chars()
        .next()
        .unwrap();

    let password = iter
        .next()
        .expect("... and ends with the password")
        .to_string();

    (min, max, char_to_check, password)
}

/**
 * The elves store passwords in their database as $min-$max $char: $pass
 *
 * Part 1:
 *	Check how many valid passwords are according to the sled rental company policy.
 *	The times $char shows up in $pass is in the range [$min, $max].
 *
 * Part 2:
 * 	Check how many valid password are according to the Official Toboggan Corporate Policy.
 * 	$char must be found at one of indices $min or $max, but not both.
 * 	Note: index starts at 1, not 0.
 */
fn main() {
    let mut valid_part_one_passwords: u32 = 0;
    let mut valid_part_two_passwords: u32 = 0;

    for line in read_lines_iter("src/day-2/input.txt") {
        let entry = line.expect("line is valid input");

        let (min, max, char_to_check, password) = split_line(entry);

        // part 1
        {
            let occurrences = password.matches(char_to_check).count() as i32;

            if (min..=max).contains(&occurrences) {
                valid_part_one_passwords += 1;
            }
        }

        // part 2
        {
            let first_index = (min - 1) as usize;
            let second_index = (max - 1) as usize;

            let first_char = password
                .chars()
                .nth(first_index)
                .expect("character at min-1 index exists");
            let second_char = password
                .chars()
                .nth(second_index)
                .expect("character at max-1 index exists");

            if (first_char == char_to_check && second_char != char_to_check)
                || (first_char != char_to_check && second_char == char_to_check)
            {
                valid_part_two_passwords += 1;
            }
        }
    }

    println!("Solution for first part: {}", valid_part_one_passwords); // 416
    println!("Solution for second part: {}", valid_part_two_passwords); // 688
}
