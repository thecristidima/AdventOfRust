use modinverse::modinverse;

use utils::files::read_lines_iter;

fn part_one(timestamp: u64, buses: &Vec<String>) -> u64 {
    let part_one_buses = buses
        .iter()
        .filter(|s| **s != "x")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut time_to_wait = u64::MAX;
    let mut best_bus = u64::MIN;

    for bus in part_one_buses {
        let leftover = timestamp % bus;

        if leftover == 0 {
            best_bus = bus;
            time_to_wait = 0;
            break;
        }

        if bus - leftover < time_to_wait {
            time_to_wait = bus - leftover;
            best_bus = bus;
        }
    }

    best_bus * time_to_wait
}

fn chinese_remainder_theorem(buses: Vec<(i128, i128)>) -> i128 {
    let a_s = buses.iter().map(|x| x.0).collect::<Vec<_>>();
    let m_s = buses.iter().map(|x| x.1).collect::<Vec<_>>();

    let m = m_s.iter().product::<i128>();

    let z_s = m_s.iter().map(|m_i| m / m_i).collect::<Vec<i128>>();

    let y_s = z_s
        .iter()
        .enumerate()
        .map(|(idx, z_i)| modinverse(*z_i, m_s[idx]).unwrap())
        .collect::<Vec<_>>();

    let w_s = y_s
        .iter()
        .zip(z_s)
        .map(|(y_i, z_i)| (y_i * z_i) % m)
        .collect::<Vec<_>>();

    a_s.iter()
        .zip(w_s)
        .map(|(a_i, w_i)| a_i * w_i)
        .sum::<i128>()
        % m
}

fn part_two(buses: &Vec<String>) -> i128 {
    let buses = buses
        .into_iter()
        .enumerate()
        .filter(|(_, val)| *val != "x")
        .map(|(idx, val)| {
            let modulo = val.parse::<i128>().unwrap();
            let remainder = modulo - idx as i128;

            (remainder, modulo)
        })
        .collect::<Vec<_>>();

    chinese_remainder_theorem(buses)
}

/**
 * Part 1:
 *  You can leave at $timestamp the earliest and each bus arrives every $busId minutes.
 *  Find the earliest bus you can take and return $busId * $minutesToWait.
 *
 * Part 2:
 *  The bus company offers you a challenge.
 *  The first bus arrives at timestamp $t, the second at $t + 1, and so on.
 *
 *  What is $t?
 *
 */
fn main() {
    let mut file_iter = read_lines_iter("src/day-13/input.txt");

    let timestamp = file_iter.next().unwrap().unwrap().parse::<u64>().unwrap();
    let buses = file_iter
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| String::from(s))
        .collect::<Vec<_>>();

    println!("Solution for part 1: {}", part_one(timestamp, &buses)); // 246
    println!("Solution for part 2: {}", part_two(&buses)); // 939490236001473
}
