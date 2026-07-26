fn main() {
    greet(15, 'A', 3.14, -5);
    println!("I am learning rust");
    scope_exp();
    let sum = fnc_that_returns(5, -8);
    println!("sum of 5 and 8 is {sum}");
    another_fn_that_returns();
}

fn another_fn_that_returns() -> u32 {
    return 5;
    println!("Hey i am running after the function returned the value, smh..");
}

fn fnc_that_returns(p1: i32, p2: i32) -> i32 {
    p1 + p2
}

fn scope_exp(){
    let y = {
        let x:u32 = 5;
        x + 1
    };
    println!("value of y is {y}");
}

fn greet(age:u32, blood_group:char, favourite_float_no:f32, favourite_int: i32) {
    println!("Heyal! I am Saksham, my age is {age}, my Blood Group is {blood_group}, idk its not neccasary though but my favourite floating no. is {favourite_float_no} and lastly, my fav integer is {favourite_int}");
}
