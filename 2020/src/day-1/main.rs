mod expenses;

/**
 * Part 1:
 * 	You get a list of expenses, find the two expenses that add up to 2020.
 * 	Return their product.
 *
 * Part 2:
 * 	Using the same list, find the _three_ expenses that add up to the same target.
 * 	Return their product.
 *
 */
fn main() {
    expenses::part_one("src/day-1/input.txt", 2020); // 921504
    expenses::part_two("src/day-1/input.txt", 2020); // 195700142
}
