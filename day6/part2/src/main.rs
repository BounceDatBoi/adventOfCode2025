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

    let mut numbers: Vec<Vec<char>> = vec![];
    let line1: Vec<char> = input
        .lines()
        .next()
        .expect("getting lines erm problems")
        .chars()
        .collect();
    for (i, line) in input.lines().enumerate() {
        if i == 4 {
            break;
        }
        let nums: Vec<char> = line.chars().collect();
        numbers.push(nums);
    }
    let mut totals: Vec<i64> = vec![];
    let mut num: Vec<&str> = vec![];

    for i in 0..4 {
        loop {
            let mut number = String::from("");
            number.push(numbers[0].pop().expect("popped"));
            number.push(numbers[1].pop().expect("popped"));
            number.push(numbers[2].pop().expect("popped"));
            number.push(numbers[3].pop().expect("popped"));
            if number.eq("    ") {
                num.push(" ");
                break;
            }
            num.push(number.as_str());
        }
        println!("num: {:?}", num);
    }
    panic!("erm");

    //for (i, op) in ops.iter().rev().enumerate() {
    //    let result = match op {
    //        '+' => numbers[0][i] + numbers[1][i] + numbers[2][i] + numbers[3][i],
    //        '*' => numbers[0][i] * numbers[1][i] * numbers[2][i] * numbers[3][i],
    //        _ => panic!("uhoh"),
    //    };
    //    totals.push(result);
    //}
    //let total: i64 = totals.iter().sum();
    //println!("total: {}", total);
}
