fn main() {
    let tuple: (u8, bool, f32) = (1, true, 0.5);
    let tuple2 = (3,4);

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);   
    println!("{:?}", tuple2);

    let (a, b, c) = tuple;

    println!("first: {} second: {} third: {}", a, b, c);
}