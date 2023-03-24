use super::Area;

pub struct Circle {
    r: f64,
}

impl Circle {
    pub fn new(r: f64) -> Self {
        Self { r }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.r * self.r * std::f64::consts::PI;
    }
}
