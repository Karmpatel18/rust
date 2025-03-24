// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
// fn main(){
//     let user = User {
//         username: String::from("someusername123"),
//         email: String::from("s9xHx@example.com"),   
//         sign_in_count: 1,
//         active: true
//     };
//     println!("{}",user.username);
// }

enum Direction {
    North,
    South,
    East,
    West
}

fn main(){
    let newdirection = Direction::North;
    moveplayer(newdirection);
    
}
    
fn moveplayer(direction:Direction){
        println!("hello ffrom mvoeplayer function",);
}
        
    
