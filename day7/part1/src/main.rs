use std::fs;

fn main() {
    println!("THISSS red_back1");
    let input = fs::read_to_string("../input.txt").expect("could not read file");

    let line: Vec<char> = vec!['.'; 143];
    let mut diagram: Vec<Vec<char>> = vec![];

    let mut diagram_output: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let char: Vec<char> = line.chars().collect();
        diagram.push(char);
    }
    diagram_output = diagram.clone();
    diagram_output.push(line.clone());
    let mut number_of_split = 0;

    for (i, line) in diagram.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if diagram_output[i][j] == '|' && diagram_output[i + 1][j] != '^' {
                diagram_output[i + 1][j] = '|';
            }
            match char {
                'S' => diagram_output[i + 1][j] = '|',
                '^' => {
                    if diagram_output[i - 1][j] == '|' {
                        diagram_output[i + 1][j - 1] = '|';
                        diagram_output[i + 1][j + 1] = '|';
                        number_of_split += 1;
                    }
                }
                _ => continue,
            };
        }
    }
    diagram_output.pop();
    for line in diagram_output {
        println!("{}", line.iter().collect::<String>());
    }
    println!("number_of_split: {number_of_split}");
}
