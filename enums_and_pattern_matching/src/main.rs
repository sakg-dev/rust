fn main() {
    // define_enum();
    // option_enum();
    // match_case();
    // if_let();
}

#[allow(dead_code, unused)]
fn if_let(){
    let max_score = Some(100u8);

    // This works but why to write _ case when we js need to handle 1 case?
    match max_score {
        Some(max) => println!("Max Score is {}", max),
        _ => ()
    }

    if let Some(max) = max_score{ // takes a pattern in left and expression in right seperated by =
        println!("Max Score is {}", max);
    }

    let i = 5;
    if let 5 = i{ // without binding
        println!("Value is 5");
    } else { // for all other cases, same as _
        println!("Value is not 5");
    }

    // here now i cvr let else..........
    // Using if let: it wrks but kind of less readability
    fn login_log_1(username_opt: Option<String>) -> String{
        if let Some(username) = username_opt {
            // all operations
            if username.len() <= 3 {
                String::from("Username needs to be longer than 3 char")
            } else {
                String::from(format!("Welcome home {}!", username))
            }
        } else {
            return String::from("No username profided wtf"); 
        }
    }
    // here its more readable but not much common as if produces a value but else returns 
    fn login_log_2(username_opt: Option<String>) -> String{
        let username = if let Some(username) = username_opt {
            // all operations
             username
        } else {
            return String::from("No username profided wtf"); 
        };

        if username.len() <= 3 {
            String::from("Username needs to be longer than 3 char")
        } else {
            String::from(format!("Welcome home {}!", username))
        }
    }
    // this is let..else.. where we are defining what to do when its not what we want rather else(we can return from there) then we can handle our specific case
    fn login_log(username_opt: Option<String>) -> String{
        let Some(username) = username_opt else {
            // println!("username: {}", username); // username is not available here
            return String::from("No username profided wtf");
        };
        // all operations
        if username.len() <= 3 {
            String::from("Username needs to be longer than 3 char")
        } else {
            String::from(format!("Welcome home {}!", username))
        }
    }
    println!("{}", login_log(None));
    println!("{}", login_log(Some(String::from("sakg"))))
}

#[allow(dead_code, unused)]
fn match_case(){
    let n = 5;
    match n{
        5 => println!("value is 5"),
        _ => println!("value is not 5"),
    }

    enum Shape {
        Circle(u32),
        Rectangle(u32, u32),
        Triangle(u32, u32),
        Square(u32)
    }
    const PI:f32 = 3.14;
    impl Shape {
        fn area(&self) -> f32 {
             match self{
                Shape::Circle(r) => PI * ((r * r) as f32),
                Shape::Rectangle(l, b) => (l * b) as f32,
                Shape::Triangle(h, b) => ((b * h) as f32)/2.0,
                Shape::Square(s) => (s * s) as f32,
            }
        }
    }
    let s1 = Shape::Circle(17).area();
    let s2 = Shape::Rectangle(23, 41).area();
    let s3 = Shape::Triangle(37, 29).area();
    let s4 = Shape::Square(18).area();
    println!("{s1}, {s2}, {s3}, {s4}");

    fn sqr(val:Option<i32>) -> Option<i32>{
        match val{
            Some(v) => Some(v*v),
            None => None,
        }
    }
    let val = sqr(Some(4));
    let val2 = sqr(None);
    println!("{}, {}", val.unwrap_or(0), val2.unwrap_or(0)); // 16 0

    fn age_comment(age:u32) -> String {
        match age {
            0 => String::from("hmm, interesting"),
            67 => String::from("hmm, u mst be fav of kids"),
            // other => String::from(format!("hmm {other} is normal.."))
            _ => String::from("uhh strange..")
        }
    }
    println!("{}", age_comment(34));

    // can also be used like:
    // match some {
    //     0 => sm_fn(),
    //     1 => sm_other_fn(),
    //     _ => ()
    // }
}

#[allow(dead_code, unused)]
fn option_enum(){
    //enum Option<T>{
    //    Some<T>,
    //    None
    //}
    let simple_int = 5;
    let strange_int1 = Some(5);
    let strange_int2:Option<u32> = None;
    println!("{}", simple_int+strange_int1.unwrap_or(0)); // 10
    println!("{}", simple_int+strange_int2.unwrap_or(0)); // 5
}

#[allow(dead_code, unused)]
fn define_enum(){
    enum Video{
        Paused,
        Playing,
        Skipping
    }

    enum Message{
        Read, // unit like varient
        Write(String), // tupple like varient
        Move{x: u32, y: u32}, // struct like varient
    }

    enum Shape{
        Circle(u32),
        Rectangle(u32, u32),
        Square(u32)
    }

    fn area(s: &Shape) {
        // same type
    }

    impl Shape{
        fn area(&self){
            println!("thhis is a method to calculate area");
        }
    }

    let s1 = Shape::Circle(7);
    let s2 = Shape::Rectangle(5, 10);
    let s3 = Shape::Square(5);
    s3.area();
}

