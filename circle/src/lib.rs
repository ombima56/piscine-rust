#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn intersect(&self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance <= (self.radius + other.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point(0.0, 0.0);
        let p2 = Point(3.0, 4.0);
        assert_eq!(p1.distance(p2), 5.0);
    }

    #[test]
    fn test_circle_new() {
        let c = Circle::new(1.0, 2.0, 3.0);
        assert_eq!(c.center, Point(1.0, 2.0));
        assert_eq!(c.radius, 3.0);
    }

    #[test]
    fn test_circle_diameter() {
        let c = Circle::new(0.0, 0.0, 10.0);
        assert_eq!(c.diameter(), 20.0);
    }

    #[test]
    fn test_circle_area() {
        let c = Circle::new(0.0, 0.0, 10.0);
        let expected_area = std::f64::consts::PI * 10.0 * 10.0;
        assert_eq!(c.area(), expected_area);
    }

    #[test]
    fn test_circle_intersect() {
        let c1 = Circle::new(0.0, 0.0, 5.0);
        let c2 = Circle::new(7.0, 0.0, 3.0);
        assert!(c1.intersect(c2)); // Should intersect

        let c3 = Circle::new(20.0, 0.0, 2.0);
        assert!(!c1.intersect(c3)); // Should not intersect
    }
}

