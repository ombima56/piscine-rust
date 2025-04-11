#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if self.r == first {
            self.r = second;
        } else if self.r == second {
            self.r = first;
        }

        if self.g == first {
            self.g = second;
        } else if self.g == second {
            self.g = first;
        }

        if self.b == first {
            self.b = second;
        } else if self.b == second {
            self.b = first;
        }

        if self.a == first {
            self.a = second;
        } else if self.a == second {
            self.a = first;
        }

        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };

        assert_eq!(c.swap(c.r, c.b), Color { r: 10, g: 200, b: 255, a: 30 });
        assert_eq!(c.swap(c.r, c.g), Color { r: 200, g: 255, b: 10, a: 30 });
        assert_eq!(c.swap(c.r, c.a), Color { r: 30, g: 200, b: 10, a: 255 });
        assert_eq!(c.swap(c.g, c.r), Color { r: 200, g: 255, b: 10, a: 30 });
    }
}
