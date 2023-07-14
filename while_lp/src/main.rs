fn main() {
    let mut i = 0;
    while i < 10 {
        i+=1;
        println!("{}", i);
        if i == 3 {
            println!("stooop");
            continue
        }
    }
}
