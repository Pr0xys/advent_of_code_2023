
use std::collections::HashMap;
fn main() {
    let input = include_str!("./part2.txt");
    let big_vector: Vec<&str> = input.split('\n').collect();
    let mut output: Vec<i32> = vec![];
    let mut aux_string: String = String::new();

    let mut hashmap_numbers: HashMap<&str, i32> =   HashMap::new();
    hashmap_numbers.insert("one", 1);
    hashmap_numbers.insert("two", 2);
    hashmap_numbers.insert("three", 3);
    hashmap_numbers.insert("four", 4);
    hashmap_numbers.insert("five", 5);
    hashmap_numbers.insert("six", 6);
    hashmap_numbers.insert("seven", 7);
    hashmap_numbers.insert("eight", 8);
    hashmap_numbers.insert("nine", 9);
    hashmap_numbers.insert("1", 1);
    hashmap_numbers.insert("2", 2);
    hashmap_numbers.insert("3", 3);
    hashmap_numbers.insert("4", 4);
    hashmap_numbers.insert("5", 5);
    hashmap_numbers.insert("6", 6);
    hashmap_numbers.insert("7", 7);
    hashmap_numbers.insert("8", 8);
    hashmap_numbers.insert("9", 9);

    for lines in &big_vector{
        let mut aux_vector: Vec<i32> = vec![];
        for ch in lines.chars(){
            aux_string.push(ch);
            for (key, value) in &hashmap_numbers {
                if aux_string.contains(key){
                    aux_vector.push(*value);
                    aux_string.clear();
                    if *key == "two" {
                        aux_string.push('o')
                    }
                    if *key == "three" 
                    || *key == "one"
                    || *key == "five"
                    || *key == "nine"{
                        aux_string.push('e')
                    }
                    if *key == "seven" {
                        aux_string.push('n')
                    }
                    if *key == "eight" {
                        aux_string.push('t')
                    } 
                }
            }
        }

        if aux_vector.is_empty(){
            output.push(0);
        }
        else if aux_vector.len() > 1  {
            output.push(aux_vector[0]*10);
            output.push(aux_vector[aux_vector.len()-1]);
            println!("{} => {:?}", lines, (aux_vector[0]*10 + aux_vector[aux_vector.len()-1]));
        } 
        else {
            output.push(aux_vector[0]*10);
            output.push(aux_vector[0]);
            println!("{} => {:?}", lines, (aux_vector[0]*10 + aux_vector[0]));
        }
    }

    let sum: i32 = output.iter().sum();
    println!("Total Sum: {}", sum);
}

