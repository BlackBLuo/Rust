trait FromStrRadixHelper:
    PartialOrd + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>
{
    const MIN: Self;
    fn from_u32(u: u32) -> Self;
    fn checked_mul(&self, other: u32) -> Option<Self>;
    fn checked_sub(&self, other: u32) -> Option<Self>;
    fn checked_add(&self, other: u32) -> Option<Self>;
}
pub fn sum(arr: &[u32]) -> Result<u32, String> {
    let s: u64 = arr.iter().map(|e| e.to_owned()).map(|e| e as u64).sum();
    if s > u32::MAX as u64 {
        return Err("over max u32".to_string());
    }
    return Ok(s as u32);
}
fn main() {
    let a: u32 ;
    let b: u32 = a.wrapping_add(1);      // 所有模式下都按照补码循环溢出规则
    println!("{b}");

    let b:Option<u32> = a.checked_add(0);
    if b.is_none(){             // 发生溢出时，返回None值。
        println!("none");
    }else{
        println!("{:?}", b.expect(""));
    }


}
