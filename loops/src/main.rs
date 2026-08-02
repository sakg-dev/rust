fn main() {
    // loop1();
    // loop2();
    // disambiguos_loop();
    // while_loop();
    for_loop()
}

fn loop1() {
    // Infinite loop incoming
    let mut idx = 0;
    loop {
        idx += 1;
        println!("Hey I will run again!");
        println!("{idx}");

        if idx >= 100{
            break;
        }

        if idx % 2 == 0 {
            continue;
        }

        println!("some operation on odd number");

        println!("-------------------")
    }
}

fn loop2() {
    let mut count = 0;
    let my_loop = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    }; // need ; not for loop but for the variable assignment
    println!("{my_loop}");
}

fn disambiguos_loop() {
    let mut count = 0;
    println!("Initializing loop");
    let my_loop = 'father: loop{
        if count >= 10 {
            println!("Loop ends");
            break 'father;
        }

        let mut remaining = 10;
        println!("Initializing nested loop");
        loop {
            if remaining == 8 {
                println!("Nested loop ended");
                break;
            }
            remaining -= 1;
        }

        count += 1;
        println!("Total count is {count}")
    };
}

fn while_loop() {
    let mut n = 40;
    while n > 0 {
        println!("current no: {n}");
        n -= 1;
    }
}

fn for_loop() {
    for n in (0..10).rev(){
        println!("{n}");
    }
}
