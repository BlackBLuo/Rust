use super::Area;

pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }
}
