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

enum Shape {
    Rectangle(f64,f64),
    Circle(f64),
}

// fn main(){
//     let newdirection = Direction::North;
//     moveplayer(newdirection);
    
// }
    
// fn moveplayer(direction:Direction){
//         println!("hello ffrom mvoeplayer function",);
// }


    fn main(){
        let rect = Shape::Rectangle(1.0,2.0);
        let circle = Shape::Circle(1.0);
    let result =    calulate_area(circle);
        println!("area of circle is {}",result);
    }

    fn calulate_area(shape:Shape) -> f64{
        let area = match shape {
            Shape::Rectangle(a,b) => a*b ,
            Shape::Circle(r)=>3.14*r*r,
            
        };
        return area;
    }
    
