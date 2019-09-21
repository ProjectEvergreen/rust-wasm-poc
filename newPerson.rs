#[derive(Debug)]
struct Person<'n> {
    name: &'n str,
    age: u8
}

fn main() {
    let name = "Aimee";
    let age = 28;
    let aimee = Person { name, age };
   // in printing out you can use `{:?}` or `{:#?}` for prettier print instead.
    println!("This is struct:\n {:#?}", aimee);
    println!("This is name: {:#?}", name);
    println!("This is age: {:#?}", age);
    //or you can print it out like this
    println!("Full Information: {:#?}, {:#?}", name, age);
}
