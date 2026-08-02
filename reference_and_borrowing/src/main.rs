use std::io;
use std::io::Write;

fn main() {
    // simple_ref();
    // mut_ref();
    // dangling_ref();
}

fn dangling_ref(){
    fn sm_int_fn() -> &u32{
        let sm_val = 5;
        sm_val
    }
    let val = sm_int_fn();
}

fn mut_ref(){
    let mut og_val:String = String::from("Hey I am SakG");
    let ref1 = &og_val;
    let ref2 = &og_val;
    let ref3 = &og_val;
    println!("{ref1}, {ref2}"); // np as readonly refs not being used now!
    let mut_ref1 = &mut og_val;
    let mut_ref2 = &mut og_val;
    // println!("{mut_ref1}"); // err
    // println!("{og_val}:{ref1}:{ref2}:{ref3}"); // problem
}

fn simple_ref(){
    fn square(n: &i32) -> i32 {
        n * n
    }

    print!("Enter the number you want to find square of: ");
    io::stdout().flush().unwrap();
    let mut ipt: String = String::new();
    io::stdin().read_line(&mut ipt).unwrap();
   
    let n = &ipt.trim().parse().unwrap();

    let squared_n = square(&n);
    println!("Square of {n} is {squared_n}");
}
