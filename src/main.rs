fn main() {
    

    stack_fn();
    heap_fn();
    stringupdate_fn();



}

fn stack_fn(){
    let x = 10;
    let y = 12;
    let z = x + y;
    println!("{}",z);
}
fn heap_fn(){
    let s1 = String::from("hello");
    let s2 = String::from("world");
    // println!("{} {}",s1,s2);
    let z = format!("{} {}",s1,s2);
    println!("{}",z);
}

fn stringupdate_fn(){
    let mut string = String::from("hello world");
    println!("{}",string);
    string.push_str("after updatation");
}

