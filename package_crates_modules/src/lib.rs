fn _my_fn(){
    println!("yo my first library");
}

// through modules, we can make our function more readable and usable like ts:
pub mod game{
    mod physics{
        pub fn calculate_collision() {}
    }
    mod communication{
        mod signal{
            pub fn add_signal() {}
            pub fn verify_signal() {}
            pub fn trigger_signal() {}
        }
        mod chat{
            mod public_chat{}
            mod admin_chat{}
        }
    }
}
