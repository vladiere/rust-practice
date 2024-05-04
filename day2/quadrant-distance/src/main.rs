// this is day 3 i made quadrant and the distance between two points;
use std::io;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_quadrant(&self) -> String {
        let mut quadrant = String::from("Quadrant ");
        if self.x > 0 && self.y > 0 {
            quadrant.push_str("1");
        } else if self.x > 0 && self.y < 0 {
            quadrant.push_str("3");
        } else if self.x < 0 && self.y > 0 {
            quadrant.push_str("2");
        } else if self.x < 0 && self.x < 0 {
            quadrant.push_str("4");
        } else {
            if self.x > 0 && self.y == 0 {
                quadrant.push_str("Y");
            } else {
                quadrant.push_str("X");
            }
        }

        quadrant
    }
}

fn get_distance(x1: i32, x2: i32, y1: i32, y2: i32) -> f32 {
    let total = ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32;
    let result = total.sqrt().abs();

    result
}

fn set_points() -> (i32, i32) {
    let mut pointx = String::new();
    let mut pointy = String::new();

    println!("Point X:");
    io::stdin()
        .read_line(&mut pointx)
        .expect("Failed to read line");

    println!("Point Y:");
    io::stdin()
        .read_line(&mut pointy)
        .expect("Failed to read line");

    let pointx: i32 = pointx.trim().parse().expect("Not a number");

    let pointy: i32 = pointy.trim().parse().expect("Not a number");

    (pointx, pointy)
}

fn main() {
    println!("Set points for point1");
    let (pointx, pointy) = set_points();

    let point1 = Point {
        x: pointx,
        y: pointy,
    };

    println!("Set points for point2");
    let (pointx, pointy) = set_points();

    let point2 = Point {
        x: pointx,
        y: pointy,
    };

    println!("The quadrant of point1 is {}\n The distance between point1 (x: {} y: {}) and point2 (x: {} y: {} is {})", point1.get_quadrant(), point1.x, point1.y, point2.x, point2.y, get_distance(point2.x, point1.x, point2.y, point1.y));
}
