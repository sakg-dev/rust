fn main() {
    // define_enum();
    // option_enum();
    match_case();
}

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
    println!("{}, {}", val.unwrap_or(0), val2.unwrap_or(0));

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
    let simple_int = 5;
    let strange_int1 = Some(5);
    let strange_int2:Option<u32> = None;
    println!("{}", simple_int+strange_int1.unwrap_or(0));
    println!("{}", simple_int+strange_int2.unwrap_or(0));
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

