fn main() {
    let input = include_str!("./part1.txt");
    let input: Vec<&str> = input.split('\n').collect();

    let mut output: usize = 0;

    for lines in input {
        lines.to_string().retain(|c| c.is_digit(10) && c == '|');
        let parts: Vec<&str> = lines.split('|').collect();
        
        let winning_numbers: Vec<i32> = parts[0].trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let own_numbers: Vec<i32> = parts[1].trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        
        let count = own_numbers.iter().filter(|&x| winning_numbers.contains(x)).count();

        if count > 0 {
            let mut points = 1;
            for _ in 1..count {
                points *= 2;
            }
            output += points;
        }
    }

    println!("{}", output)
    
}