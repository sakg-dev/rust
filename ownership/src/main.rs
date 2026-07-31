fn main() {
    let my_str_literal = "Hellow world";
    let my_str:String = String::from("My dynamic string");
    println!("{}:{}", my_str_literal, my_str);
    
    let new_str_literal = my_str_literal;
    println!("{new_str_literal}:{my_str_literal}");

    // let new_str = my_str;
    let new_str = my_str.clone();
    println!("{new_str}, {my_str}");

    take_ownership(my_str);
    // println!("{my_str}"); // val has already been freed

    let data:String = String::from("Hellowwwww");
    let (length, data) = take_ownership_and_return(data);
    println!("data var was defined and gave its ownership to a func and the func returned the var's value and its additional data. val: {}, length: {}", data, length);
}

fn take_ownership(data:String){
    println!("{data}");
}


fn take_ownership_and_return(data: String) -> (usize, String){
    (data.len(), data)
}
