use regex::Regex;

fn main() {
    let input = include_str!("./part2.txt");
    let input: Vec<&str> = input.split('\n').collect();
    let mut output:i32 = 0;

    for lines in input {
        output += check_colors(lines);
    }
    
    println!("{}", output);
}


fn check_colors(line: &str) -> i32 {
    let reggie = Regex::new(r"(\d+)\s*(blue|red|green)").unwrap();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for capz in reggie.captures_iter(line) {
        let number: i32 = capz[1].parse().unwrap_or(0);
        match &capz[2] {
            "red" => max_red = max_red.max(number),
            "green" => max_green = max_green.max(number),
            "blue" => max_blue = max_blue.max(number),
            _ => (),
        }
    }
    println!("The total red: {}, the total green: {}, the total blue: {}", max_red, max_green, max_blue);

    let power_set = max_red * max_green * max_blue;
    
    power_set
}