fn main() {
    let input = include_str!("./part1.txt");
    let input: Vec<&str> = input.split("\n\n").collect();
    println!("{:?}", input);
 
}