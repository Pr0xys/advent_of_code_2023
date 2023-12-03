/* 
    Disclaimer: This solution is a rewrite into Rust,
    orignal found on: https://github.com/encse/adventofcode/blob/master/2023/Day03/Solution.cs


*/


use regex::Regex;

fn main() {
    let input = include_str!("part2.txt");
    let rows: Vec<&str> = input.split('\n').collect();

    let total = find_total(&rows);
    println!("Total: {}", total);
}

fn find_total(rows: &[&str]) -> i32 {
    let gears = parse(rows, &Regex::new(r"\*").unwrap());
    let numbers = parse(rows, &Regex::new(r"\d+").unwrap());

    gears.iter()
        .filter_map(|g| {
            let neighbours: Vec<i32> = numbers.iter()
                .filter(|&n| find_next_to(n, g))
                .map(|n| n.value())
                .collect();

            if neighbours.len() == 2 {
                Some(neighbours[0] * neighbours[1])
            } else {
                None
            }
        })
        .sum()
}

fn parse(rows: &[&str], regex: &Regex) -> Vec<Part> {
    rows.iter()
        .enumerate()
        .flat_map(|(i_row, &row)| {
            regex.find_iter(row)
                 .map(move |mat| Part {
                     text: mat.as_str().to_string(),
                     i_row,
                     i_col: mat.start(),
                 })
        })
        .collect()
}

fn find_next_to(p1: &Part, p2: &Part) -> bool {
    (p2.i_row as i32 - p1.i_row as i32).abs() <= 1 &&
    p1.i_col <= p2.i_col + p2.text.len() &&
    p2.i_col <= p1.i_col + p1.text.len()
}

#[derive(Debug)]
struct Part {
    text: String,
    i_row: usize,
    i_col: usize,
}

impl Part {
    fn value(&self) -> i32 {
        self.text.parse::<i32>().unwrap_or(0)
    }
}
