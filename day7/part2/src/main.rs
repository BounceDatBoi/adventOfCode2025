use std::fs;

fn main() {
    println!("xdx |");
    let input = fs::read_to_string("../input.txt").expect("could not read file '../input.txt'");

    let mut beams: Vec<i64> = vec![0; input.lines().next().expect("input file empty").len()];

    for line in input.lines() {
        let mut new_beams = beams.clone();
        let mut split_occured = false;

        for (i, char) in line.chars().enumerate() {
            match char {
                'S' => beams[i] = 1,
                '^' => {
                    split_occured = true;

                    let splits = beams[i];

                    if i > 0 {
                        new_beams[i - 1] += splits;
                    }
                    if i < beams.len() - 1 {
                        new_beams[i + 1] += splits;
                    }

                    new_beams[i] = 0;
                }
                _ => continue,
            }
        }

        if split_occured {
            beams = new_beams;
        }
    }

    let total_timelines: i64 = beams.iter().sum();
    println!("Total number of timelines: {}", total_timelines);
}
