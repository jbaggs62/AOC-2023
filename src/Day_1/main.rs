use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn main()
{
    let lines_of_text = lines_from_file("src/Day_1/input.txt");
    let clean_strings = processor(lines_of_text);
    let combined_strings = convert_to_integers(clean_strings);
    let total = sum_values(combined_strings);
    println!("Sum: {}", total)

}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn processor(input: Vec<String>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for string in input {
        let mut filtered_chars: Vec<char> = Vec::new();

        for char in string.chars() {
            if char.is_digit(10) {
                filtered_chars.push(char);
            }
        }

        let mut first_last: Vec<char> = Vec::new();
        if let Some(first) = filtered_chars.first().cloned() {
            first_last.push(first);
            if filtered_chars.len() > 1 {
                if let Some(last) = filtered_chars.last().cloned() {
                    first_last.push(last);
                }
            }
        }

        result.push(first_last); // Store the first and last characters in the result
    }

    result
}

fn convert_to_integers(input: Vec<Vec<char>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for chars in input {
        let string: String = chars.into_iter().collect();
        if let Ok(number) = string.parse::<i32>() {
            result.push(number);
        }
    }

    result
}
fn sum_values(values: Vec<i32>) -> i32 {
    values.iter().sum()
}






