#![allow(dead_code)]
use std::time::Duration;

/// 交通灯，每个交通灯都会指定自己的阻塞时间
pub enum Light {
    RED(u64),
    YELLOW(u64),
    BLUE(u64),
}

/// 一个抽象的阻塞trait，通过一个方法获知阻塞时间
pub trait Blocker {
    /// 获取阻塞时间
    fn block_time(&self) -> Duration;
}

/// 为[`Light`]实现[`Blocker`] `trait`
/// ```rust
/// fn main() {
///     let red = Light::RED(3);
///     println!("red blocker : {:?}", red.block_time());
/// }
/// ```
impl Blocker for Light {
    fn block_time(&self) -> Duration {
        match &self {
            Light::RED(t) => Duration::from_secs(t.to_owned()),
            Light::YELLOW(t) => Duration::from_secs(t.to_owned()),
            Light::BLUE(t) => Duration::from_secs(t.to_owned()),
        }
    }
}
