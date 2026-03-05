#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_palindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}

fn check_line(vec: &[char], i: usize, character: char) -> bool {
    let prev_char = vec.get(i - 1).unwrap();
    let curr_char = vec.get(i).unwrap();
    let next_char = vec.get(i + 1).unwrap();

    *prev_char == character && *curr_char == character && *next_char == character
}

fn main() {
    let file = File::open("./symbole.txt").unwrap();
    let reader = BufReader::new(file);

    let mut palindrome_count = 0;

    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();

        if is_palindrome(line) {
            println!("{line}");
            palindrome_count += 1;
        }

        vec.push(line.chars().collect());
    }

    for (i, window) in vec.windows(3).enumerate() {
        let prev_line = &window[0];
        let line = &window[1];
        let next_line = &window[2];

        for j in 1..line.len() - 1 {
            let character = line[j];
            let ch1 = check_line(prev_line, j, character);
            let ch2 = check_line(line, j, character);
            let ch3 = check_line(next_line, j, character);

            if ch1 && ch2 && ch3 {
                println!("({},{})", i + 2, j + 1);
            }
        }
    }

    let mut total = 0;
    let mut biggest = 0;

    for line in vec {
        let value = line
            .iter()
            .map(|char| match char {
                'o' => '0',
                '+' => '1',
                '*' => '2',
                _ => unreachable!(),
            })
            .collect::<String>();

        let mut result = 0;
        for c in value.chars() {
            result = result * 3 + c.to_digit(3).unwrap();
        }

        total += result;
        if result > biggest {
            biggest = result;
        }
    }

    println!("Palindrome count: {palindrome_count}");
    println!("Biggest: {biggest}");
    println!("Total: {total}");
}
