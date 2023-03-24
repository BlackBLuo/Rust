mod circle;
mod square;
mod triangle;

pub trait Area {
    fn area(&self) -> f64;
}

pub use circle::*;
pub use square::*;
pub use triangle::*;
