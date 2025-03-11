fn main() {
    println!("Hello, world!");
    let greeting = String::from("Hello");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(3);
    match char1 {
        Some(c) => println!("{}",c),
        None => println!("none"), 
        
    }
    // println!("{}",char1.unwrap());
}
