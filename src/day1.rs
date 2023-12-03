// --- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
// For example:
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

use std::{collections::HashMap, char};

use crate::aoc::day::AdventOfCodeDay;

fn parse_digits(first_digit: i32, last_digit: i32) -> i32 {
    let full_digit_str = format!("{}{}", first_digit, last_digit);
    let full_digit = full_digit_str.parse::<i32>().unwrap();
    return full_digit
}

pub fn part1(day: &AdventOfCodeDay) {
    let input = day.get_input::<Vec<String>>(Some(1));
    let mut sum = 0;

    for line in input {
        let chars = line.chars();
        let mut first_digit: Option<i32>= None;
        let mut last_digit: Option<i32> = None;

        let numbers = chars
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        if numbers.len() > 0 {
            for i in 0..numbers.len() {
                if i == 0 {
                    first_digit = Some(numbers[i] as i32);
                }

                if i == numbers.len() - 1 {
                    last_digit = Some(numbers[i] as i32);
                }
            }
        }

        sum += parse_digits(first_digit.unwrap(), last_digit.unwrap())
    }

    println!("Part 1: {}", sum);
}

pub fn part2(day: &AdventOfCodeDay) {
    let valid_word_digits = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let word_to_digit_hash: HashMap<String, i32> = valid_word_digits
        .iter()
        .enumerate()
        .map(|(i, s)| (s.to_string(), i as i32 + 1))
        .collect();

    let input = day.get_input::<Vec<String>>(Some(2));
    let mut sum = 0;

    for line in input {
        let chars = line.chars();
        let mut first_digit = None;
        let mut last_digit = None;

        let letters = chars
            .filter(|c| c.is_alphabetic() || c.is_digit(10))
            .collect::<Vec<char>>();

        if letters.len() > 0 {
            let mut index = 0;

            while index < letters.len() {
                let char = letters[index];

                if char.is_digit(10) {
                    if first_digit == None {
                        first_digit = Some(char.to_digit(10).unwrap() as i32);
                    } else {
                        last_digit = Some(char.to_digit(10).unwrap() as i32);
                    }
                    index += 1;
                    continue;
                } 
                
                // ignore standalone letters as they are not valid words
                if char.is_alphabetic() && letters[index].is_alphabetic() && index + 1 < letters.len() && letters[index + 1].is_alphabetic() {
                    let mut word = String::new();
                    let mut search_index = index;

                    // sub loop through letters to find a valid word
                    while search_index < letters.len() && letters[search_index].is_alphabetic() {
                        word.push(letters[search_index]);

                        if word_to_digit_hash.contains_key(&word) {
                            if first_digit == None {
                                first_digit = match word_to_digit_hash.get(&word) {
                                    Some(digit) => Some(*digit),
                                    None => None,
                                };
                            } else {
                                last_digit = match word_to_digit_hash.get(&word) {
                                    Some(digit) => Some(*digit),
                                    None => None,
                                };
                            }
                            // Skip index to the end of the word - words can share letters
                            index += word.len() - 2;
                            break
                        } 
                        search_index += 1;
                    }
                }
                index += 1;
            }
        } 

        let mut value = 0;

        if first_digit.is_some() && last_digit.is_some() {
            value = parse_digits(first_digit.unwrap(), last_digit.unwrap());
        } else if first_digit.is_some() {
            // some lines only have one digit
            // use the first digit as the last digit in this case
            value = parse_digits(first_digit.unwrap(), first_digit.unwrap());
            println!("Only one digit: {}", value);
        }         

        sum += value;
    }
    println!("Part 2: {}", sum);
}