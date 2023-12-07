fn main() {
    let input = include_str!("./part1.txt");
    let input: Vec<&str> = input.split("\n").collect();
    println!("{:?}", input);

    let time:String = input[0].chars().filter(|x| x.is_digit(10)).collect();
    let time:u64 = time.trim().parse().unwrap();
    
    let distance:String = input[1].chars().filter(|x| x.is_digit(10)).collect();
    let distance:u64 = distance.trim().parse().unwrap();

    let mut output: u64 = 0;
    for i in 1.. time{
        if i * (time - i) > distance{
            output += 1;
        }
    }
    
    println!("total: {}", output)
}