use std::error::Error;
use std::fs;

/// ---------- PART 1 ----------
fn is_repeated_twice_exact(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let half = s.len() / 2;
    let (a, b) = s.split_at(half);
    a == b
}

pub fn solve_part_one(input: &str) -> u128 {
    let mut total: u128 = 0;

    for range in input.trim().split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for id in start..=end {
            if is_repeated_twice_exact(&id.to_string()) {
                total += id as u128;
            }
        }
    }

    total
}

/// ---------- PART 2 ----------
/// Prüft, ob s aus einem Chunk besteht, der *mindestens zweimal* wiederholt wird.
///
/// Beispiele:
/// "55" -> true
/// "111" -> true (Chunk "1", 3 Wiederholungen)
/// "123123" -> true (Chunk "123" x2)
/// "ababab" -> true (Chunk "ab" x3)
fn is_repeated_at_least_twice(s: &str) -> bool {
    let len = s.len();

    // für jede mögliche chunk-Länge
    for chunk_len in 1..=len/2 {
        if len % chunk_len != 0 {
            continue;
        }

        let repeats = len / chunk_len;
        if repeats < 2 {
            continue;
        }

        let chunk = &s[0..chunk_len];

        if chunk.repeat(repeats) == s {
            return true;
        }
    }

    false
}

pub fn solve_part_two(input: &str) -> u128 {
    let mut total: u128 = 0;

    for range in input.trim().split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for id in start..=end {
            if is_repeated_at_least_twice(&id.to_string()) {
                total += id as u128;
            }
        }
    }

    total
}


/// ---------- MAIN ----------
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    // Part 1:
    // let answer = solve_part_one(&input);

    // Part 2:
    let answer = solve_part_two(&input);

    println!("Answer: {}", answer);
    Ok(())
}
