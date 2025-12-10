use std::fs;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn parse_new(line: &str) -> Self {
        let mut number = line.split(',');
        let x = number
            .next()
            .expect("erm error")
            .parse::<i64>()
            .expect("parsing error");

        let y = number
            .next()
            .expect("erm error")
            .parse::<i64>()
            .expect("parsing error");
        Self::new(x, y)
    }

    fn rectangle_area(&self, point: Point) -> i64 {
        (point.x - self.x + 1) * (point.y - self.y + 1)
    }
}

fn main() {
    println!("Stick Chimera7");

    let input = fs::read_to_string("../input.txt").expect("erm is it there ?");
    //let input = fs::read_to_string("../example.txt").expect("erm is it there ?");

    let mut max_x = 0;
    let mut max_y = 0;

    let mut points: Vec<Point> = vec![];

    for line in input.lines() {
        points.push(Point::parse_new(line));
    }

    for point in points.clone() {
        if point.x > max_x {
            max_x = point.x
        }

        if point.y > max_y {
            max_y = point.y
        }
    }

    let mut max_area: i64 = 0;
    for point in points.clone() {
        for p in points.clone() {
            let area = point.rectangle_area(p);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("max_area: {max_area}");
}
