use regex::Regex;

fn main() {
    let input = include_str!("./part1.txt");
    let input: Vec<&str> = input.split('\n').collect();

    let mut counter: i32 = 1;
    let mut output:i32 = 0;

    for lines in input {
        
        if check_color(lines){
            output += counter;
        }
        counter +=1;
    }

    println!("{} and final counter: {}", output, counter)

}

fn check_color(line: &str) -> bool {
    let reggie = Regex::new(r"(\d+)\s*(blue|red|green)").unwrap();

    for capz in reggie.captures_iter(line) {
        let number: i32 = capz[1].parse().unwrap_or(0);
        match &capz[2] {
            "red" if number > 12 => return false,
            "green" if number > 13 => return false,
            "blue" if number > 14 => return false,
            _ => (),
        }
    }
    true
}
