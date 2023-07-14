fn main() {
    let str : &str = "hello rust";
    let mut string : String = String::from("Hello rust");
    
    println!("{}", string);

    let slice = &string[6 .. 10];
    
    println!("{}", slice);
    println!("{}", slice.len());

    string.push('1');
    println!("{}", string);

    string.push_str("! zuka");
    println!("{}", string);

    string = string.replace("!", " Hola");
    println!("{}", string);

}
