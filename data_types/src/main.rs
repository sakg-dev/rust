fn main() {
    let guess:u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let n_no:i8 = -5;

    let p_no:u8 = 5;

    let f_no = 3.14;

    let u_f_no:u32 = 22/7;
    println!("{u_f_no}");

    let add = 5 + -5;
    let sub = 5 - 8;
    let mul = 5 * -8;
    let div = 5 / 8;
    let truncated = -5 / 3;
    let remainder = 5 % 3;

    println!("add: {add}");
    println!("sub: {sub}");
    println!("mul: {mul}");
    println!("div: {div}");
    println!("truncated: {truncated}");
    println!("remainder: {remainder}");

    let is_true:bool = false;
    println!("isTrue: {is_true}");

    let is_it_string:char = 'e';
    println!("is_it_string: {is_it_string}");

    let tup: (char, u32, f32, bool, (i32, char)) = ('A', 54, 25.78, true, (-5, 'B'));
    println!("{}", tup.1);
    let (tup_char, unsigned32, floating32, boolean, nested_tupple) = tup;
    println!("{tup_char}, {unsigned32}, {floating32}, {boolean}");
    let (signed_int, character) = nested_tupple;
    println!("{signed_int}, {character}");
}
