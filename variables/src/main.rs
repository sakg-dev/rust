fn main() {
    const USER_AGE_IN_SECONDS:u32 = 15 * 365 * 24 * 60 * 60; 
    let mut name = "sak";
    name = "sakg";
    println!("{name} is {USER_AGE_IN_SECONDS} seconds old");
    let x = 0;
    println!("x first value: {x}");
    let x = x + 5;
    println!("x second val: {x}");
    {
        let x = x * x;
        println!("x inside scope: {x}");
    }

    println!("x final value: {x}");

    let surname = "goswami";
    println!("My surname is {surname}");
    let surname = surname.len();
    println!("its length is {surname}");
}
