/* --- Part Two ---
Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.

Or are they?

The Historians can't agree on which group made the mistakes or how to read most of the Chief's handwriting, but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists! Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.

This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

Here are the same example lists again:

3   4
4   3
2   5
1   3
3   9
3   3
For these example lists, here is the process of finding the similarity score:

The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
The fourth number, 1, also does not appear in the right list.
The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
The last number, 3, appears in the right list three times; the similarity score again increases by 9.
So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).

Once again consider your left and right lists. What is their similarity score? */

use std::collections::HashMap;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the input file
    
    let input_path = "input1.txt";

    // Read and parse the input file
    let (left_list, right_list) = parse_input(input_path)?;

    // Compute the similarity score
    let similarity_score = calculate_similarity_score(&left_list, &right_list);

    // Output the result
    println!("Similarity Score: {}", similarity_score);

    Ok(())
}
//meg a l´fasy
// Parse the input file into two lists of integers
fn parse_input(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in content.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        } else {
            return Err("Invalid line format in input".into());
        }
    }

    Ok((left_list, right_list))
}

// Compute the similarity score
fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i32 {
    // Count occurrences of each number in the right list
    let mut right_counts = HashMap::new();
    for &num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    left.iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum()
}
