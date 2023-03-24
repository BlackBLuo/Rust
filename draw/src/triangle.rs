use super::Area;

pub struct Triangle {
    sides: [f64; 3],
    hight: f64,
    bottom: f64,
}

impl Triangle {
    pub fn new(sides: [f64; 3], hight: f64, bottom: f64) -> Self {
        Self {
            sides,
            hight,
            bottom,
        }
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        return self.hight * self.bottom;
    }
}
