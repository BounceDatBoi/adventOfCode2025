use std::fs;

fn main() {
    println!("fu red_back1");

    let input = fs::read_to_string("input.txt").expect("Couldn't read the file");

    let ops: Vec<char> = input
        .lines()
        .last()
        .expect("error getting the last line")
        .replace(" ", "")
        .chars()
        .collect();

    let mut numbers: Vec<Vec<i64>> = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 4 {
            break;
        }
        let nums: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().expect("i64 parsing error"))
            .collect();
        numbers.push(nums);
    }
    let mut totals: Vec<i64> = vec![];

    for (i, op) in ops.iter().enumerate() {
        let result = match op {
            '+' => numbers[0][i] + numbers[1][i] + numbers[2][i] + numbers[3][i],
            '*' => numbers[0][i] * numbers[1][i] * numbers[2][i] * numbers[3][i],
            _ => panic!("uhoh"),
        };
        totals.push(result);
    }
    let total: i64 = totals.iter().sum();
    println!("total: {}", total);
}
