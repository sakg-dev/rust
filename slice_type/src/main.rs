// &str is js a refernce of uft-8 text, doesn't care where it is coming from.. it contains pointer and length by which it can also calculate the end idx kinda.. we can pass &String(gets automatically cnvrted to &str using deref coercions feature) and &str both.

fn main() {
    let mut string = String::from("thousand years zellij rust bedwar lol");
    // let first_word = bad_find_first_word(&string);
    // first_word_without_slice(&mut string);
    
    let first_w = first_word(&string);
    println!("{}", first_w);

    do_arr_slice(&[5,6,8,4,2]);
}

fn do_arr_slice(arr:&[i32;5]) {
    let new_arr = &arr[..2];
    for item in new_arr{
        println!("{}", item);
    }
}


fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..idx];
        }
    }
    return &s[..];
}

fn first_word_without_slice(s: &mut String) {
    fn get_len_of_first_space(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (idx, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                println!("Space found at {}", idx);
                return idx;
            }
        }
        return s.len();
    }

    let f = get_len_of_first_space(s);
    s.clear();
    println!("{}:{}", f, s); // f is still pointing to an idx that is invalid :(
}

fn bad_find_first_word(s: &String) -> String {
    s.split(" ").nth(0).expect("aaa").to_string()
}

