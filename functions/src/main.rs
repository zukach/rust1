fn main(){
    println!("{}", is_even(7))
}

fn is_even(num: u8) -> bool{
    let digit: u8 = num%2;
    digit == 0
}