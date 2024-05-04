#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Quadrant {
    point: Point,
}

#[derive(Debug, Clone)]
struct Line {
    point1: Point,
    point2: Point,
}

impl Quadrant {
    fn new(point: Point) -> Self {
        Self { point }
    }
    fn check_quadrant(self) -> String {
        if self.point.x > 0.0 && self.point.y > 0.0 {
            format!(
                "x: {} and y: {} - is in Quadrant 1",
                self.get_x(),
                self.get_y()
            )
        } else if self.point.x < 0.0 && self.point.y > 0.0 {
            format!(
                "x: {} and y: {} - is in Quadrant 2",
                self.get_x(),
                self.get_y()
            )
        } else if self.point.x > 0.0 && self.point.y < 0.0 {
            format!(
                "x: {} and y: {} - is in Quadrant 4",
                self.get_x(),
                self.get_y()
            )
        } else {
            format!(
                "x: {} and y: {} - is in Quadrant 3",
                self.get_x(),
                self.get_y()
            )
        }
    }

    fn get_x(&self) -> String {
        format!("{}", &self.point.x)
    }

    fn get_y(&self) -> String {
        format!("{}", &self.point.y)
    }
}

impl Line {
    fn new(point1: Point, point2: Point) -> Self {
        Self { point1, point2 }
    }
    fn point_distance(self) -> String {
        let result = ((self.point1.x - self.point2.x).powf(2.0)
            + (self.point1.y - self.point2.y).powf(2.0))
        .sqrt();
        format!(
            "Line distance between Point 1 x: {} and y: {}, Point 2 x: {} and y: {} is equal: {}",
            self.point1.x, self.point1.y, self.point2.x, self.point2.y, result
        )
    }

    fn pythagorean(&self) -> String {
        let a = &self.point1.y - &self.point2.y;
        let b = &self.point1.x - &self.point2.x;
        let c = a + b;

        format!("The length of the segment is {}", c)
    }
}

fn main() {
    let point = Point { x: 2.0, y: -3.0 };
    let quadrant1 = Quadrant::new(point);

    println!("{}", quadrant1.check_quadrant());

    let point1 = Point { x: 3.3, y: 5.0 };
    let point2 = Point { x: 0.5, y: -56.2 };
    let line1 = Line::new(point1, point2);

    println!("{}", line1.clone().point_distance());
    println!("{}", line1.clone().pythagorean());
}
