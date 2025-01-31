use std::{f64::EPSILON, ops::Deref, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn cross_product(p1: Point, p2: Point, p3: Point) -> f64 {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

struct Polygon(Vec<Point>);

impl Deref for Polygon {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Polygon {
    fn check_point(&self, point: Point) -> bool {
        if self.len() < 3 {
            return false;
        }

        let mut sign = None;
        for i in 0..self.len() {
            let p1 = self[i];
            let p2 = self[(i + 1) % self.len()];
            let cross = cross_product(p1, p2, point);

            if cross.abs() < EPSILON {
                continue;
            }

            let current_sign = cross > 0.0;
            if let Some(prev_sign) = sign {
                if prev_sign != current_sign {
                    return false;
                }
            } else {
                sign = Some(current_sign);
            }
        }
        return true;
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = s.split_whitespace().into_iter();
        let x = value
            .next()
            .ok_or("not found x")?
            .parse::<f64>()
            .map_err(|_| "could not parse x")?;

        let y = value
            .next()
            .ok_or("not found y")?
            .parse::<f64>()
            .map_err(|_| "could not parse y")?;

        return Ok(Point { x, y });
    }
}

impl From<String> for Polygon {
    fn from(value: String) -> Self {
        let mut data = Vec::<Point>::new();
        for line in value.lines() {
            data.push(Point::from_str(line).unwrap());
        }
        Self(data)
    }
}
fn main() {
    let polygon = Polygon::from(std::fs::read_to_string("./file").unwrap());

    // let test_point = Point { x: 5.0, y: 5.0 }; // false
    let test_point = Point { x: 2.0, y: 2.0 }; // true

    println!(
        "Is point inside polygon? {}",
        polygon.check_point(test_point)
    );
}
