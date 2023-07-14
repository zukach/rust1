fn main() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C{x:10, y:20};
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",c);

    if let MyEnum::B(val) = b{
        println!("{}", val)
    }

    if let MyEnum::C{x,y} = c{
        println!("{} {}", x, y)
    }
}

#[derive(Debug)]
enum MyEnum{
    A,
    B(i32),
    C {x: i32, y:i32}
}


//second Enum example
// enum Color {
//     Red,
//     Green,
//     Blue,
//     RGB(u8, u8, u8), // Variant with associated values
// }

// fn main() {
//     let favorite_color = Color::RGB(45, 112, 43);

//     match favorite_color {
//         Color::Red => println!("The color is red!"),
//         Color::Green => println!("The color is green!"),
//         Color::Blue => println!("The color is blue!"),
//         Color::RGB(r, g, b) => {
//             println!("The color is an RGB value: {}, {}, {}", r, g, b);
//         }
//     }
// }