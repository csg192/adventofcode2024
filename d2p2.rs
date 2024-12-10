/* --- Part Two ---
The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

7 6 4 2 1: Safe without removing any level.
1 2 7 8 9: Unsafe regardless of which level is removed.
9 7 6 2 1: Unsafe regardless of which level is removed.
1 3 2 4 5: Safe by removing the second level, 3.
8 6 4 4 1: Safe by removing the third level, 4.
1 3 6 7 9: Safe without removing any level.
Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe? */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i-1];
        if diff <= 0 {
            increasing = false;
        }
        if diff >= 0 {
            decreasing = false;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    increasing || decreasing
}

fn is_safe_with_dampener(levels: &Vec<i32>) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        if is_safe(&modified_levels) {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let path = Path::new("day2.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if is_safe_with_dampener(&levels) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports with Problem Dampener: {}", safe_count);
    Ok(())
}
