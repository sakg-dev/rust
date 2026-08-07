fn main() {
    // define_enum();
    option_enum();
}

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

