fn main() {
    let input = include_str!("./part2.txt");
    let input: Vec<&str> = input.split('\n').collect();

    let mut numbers:Vec<(Vec<i32>, Vec<i32>)> = Vec::new();

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
        
            numbers.push((winning_numbers, own_numbers));
    }

    let mut num_cards = vec![1; numbers.len()];

    for (i, card) in numbers.iter().enumerate() {
        let (winning, own) = card;
        let matching_numbers = own.iter().filter(|&&num| winning.contains(&num)).count();

        for j in i + 1..std::cmp::min(i + 1 + matching_numbers, numbers.len()) {
            num_cards[j] += num_cards[i];
        }
    }

    let total_cards: usize = num_cards.iter().sum();
    println!("{}", total_cards);
    
}