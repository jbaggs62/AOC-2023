use std::collections::HashMap;

fn main()
{
   println!("Hello-World");
   processor();
   processor_pt2();
}

const INPUT: &str = include_str!("input.txt");
fn processor (){
    let digits: Vec<String> = INPUT.lines().map(|x| x.chars().filter(|x| x.is_numeric()).collect()).collect();

    let mut sum = 0;
    for digit in digits {
        let characters: Vec<char> = digit.chars().collect();
        if characters.len() >= 1 {
            sum += format!("{}{}", characters[0], characters[characters.len() -1]).parse::<i32>().unwrap();
        }
    }

    println!("{}", sum)
}




fn processor_pt2() {
    let mut number_map = HashMap::new();
    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);

    const INPUT: &str = include_str!("input.txt");

    let mut replaced_string = String::from(INPUT);

    for (word, &val) in &number_map {
        replaced_string = replaced_string.replace(word, &val.to_string());
    }

    let digits: Vec<&str> = replaced_string.split_whitespace().collect(); // Splitting the replaced string into individual words

    let mut sum = 0;
    for digit in &digits {
        let characters: Vec<char> = digit.chars().collect();
        if characters.len() >= 1 {
            sum += format!("{}{}", characters[0], characters[characters.len() - 1])
                .parse::<i32>()
                .unwrap_or(0); // Handling if parsing fails
        }
    }

    println!("Sum of parsed digits: {}", sum);
}





