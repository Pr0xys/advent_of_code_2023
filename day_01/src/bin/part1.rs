fn main() {
    let input = include_str!("./part1.txt");
    let big_vector: Vec<&str> = input.split('\n').collect();
    let mut output: Vec<i32> = vec![];

    for lines in &big_vector {
        let mut first_digit: Option<i32> = None;
        let mut last_digit: Option<i32> = None;

        for ch in lines.chars() {
            if ch.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(ch.to_digit(10).unwrap() as i32);
                }
                last_digit = Some(ch.to_digit(10).unwrap() as i32);
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let combined_value = first * 10 + last;
            output.push(combined_value);
        }
    }

    let sum: i32 = output.iter().sum();
    println!("{}", sum);
}