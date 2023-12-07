fn main() {
    let input = include_str!("./part1.txt");
    let input: String = input.split("\n").collect();
    let input: String = input.chars().filter(|x| x.is_digit(10) || x == &' ').collect();
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", input);

    let mut first_race: i32 = 0;
    let mut second_race: i32 = 0;
    let mut third_race: i32 = 0;
    let mut forth_race: i32 = 0;

    for i in 1..input[0]{
        if i * (input[0] - i) > input[4] {
            first_race += 1;
        }
    }
    println!("first race: {}" , first_race);

    for i in 1.. input[1]{
        if i * (input[1] - i) > input[5]{
            second_race += 1;
        }
    }

    println!("second race: {}" , second_race);

    for i in 1..input[2]{
        if i * (input[2] - i) > input[6]{
            third_race += 1;
        }
    }

    println!("third race: {}" , third_race);

    for i in 1..input[3] {
        if i * (input[3] - i) > input[7]{
            forth_race += 1;
        }
    }

    println!("third race: {}" , forth_race);

    println!("Answer: {}" , first_race * second_race * third_race * forth_race)
}