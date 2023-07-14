fn divide(dividend: i32, divisor: i32) -> Option<i32>{
    if dividend % divisor != 0{
        None
    } else{
        Some(dividend / divisor)
    }

}
fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    //causes Panic
    println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}
