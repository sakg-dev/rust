fn main() {
    conditioning();
    age_verification();
    one_line_conditioning(false);
}

fn conditioning(){
    let val = 4;
    if val >= 5 {
        println!("Value is bigger or equal than 5");
    } else {
        println!("Value is smaller than 5")
    }

//    if val {
//        println!("I am js dev")
//    }
}

fn age_verification() {
    let age:i32 = 16;
    let message;
    if age < 0 {
        message = "this is not possible unless u are a liar!!";
    } else if age <= 5 {
        message = "Your parents need to be informed that u are getting too much brainrot";
    } else if age <= 18 {
        message = "Yo, we lowkey are of same category!";
    } else if age <= 100 {
        message = "Hey outdated person!";
    } else {
        message = "I am worried about you :(";
    }

    println!("{message}");
}

fn one_line_conditioning(full_name:bool) {
    let name = if full_name { "Saksham" } else { "SakG" };
    println!("My name is {name}");
    // let mismatched = if true { 5 } else { "SakG" };
}
