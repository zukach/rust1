fn main() {
    let name = String::from("Zuka");
    let bird = Bird {name, attack: 10};
    bird.print_name();
    bird.print_attack();
    println!("{} {}", bird.can_fly(), bird.is_animal());
}

struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn print_name(&self){
        println!("{}", self.name);
    }
    fn print_attack(&self){
        println!("{}", self.attack);
    }
}

impl Animal for Bird{
    fn can_fly(&self) -> bool{
        true
    }
    fn is_animal(&self) -> bool{
        false
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        true
    }
}
