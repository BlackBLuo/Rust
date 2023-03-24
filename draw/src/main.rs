use std::ops::Mul;  
use std::convert::{Into};
struct Rect<T, U>   // 为结构体添加两个泛型
{
    width: T,       // 宽和高是不同的泛型
    height: U
}

struct Circle<T, U> 
{
    radius: T,
}

impl<T, U> Rect<T, U> {   
    fn area(&self) -> T     
    where T: Mul<Output = T> + Copy,     
          U: Into<T> + Copy {
        self.width.mul(self.height.into())
    }
}


fn main() {
    let rect = Rect{width:3, height:4};
    let circle = Circle{radius: 5}
    println!("{}", rect.area(),circle.area());
}
