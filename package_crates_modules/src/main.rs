
// mod renderer
// {
//
// }

pub mod renderer; // the pub keyword lets us use renderer externally, making the mod public doesn't make its contents public
use std::{io, io::Write}; // use io and io::Write
// use std::io::{self, Write}; // same like above, but uses self..
// use std::*; // bring everything which is in std, here in scope.

fn main() {
    println!("Hello, world!");

    renderer::print_smthing();
    // renderer::very_humble_task(); // can't do as it is private..

    let renderer = renderer::Renderer {
        fps: 16,
        quality: renderer::Quality::Fine
    };
    println!("{:?}", renderer);

    abt_path();

    public_private_data();
}

fn root_out() {

}
mod smthing {
    fn out() {

    }
    pub mod smthing_other{
        pub fn print_path_type(path_type:&str) {
            println!("using {} path!!", path_type);
            super::out(); // super is for accessing parent path, js like ../
            super::super::root_out() // one super for going out of smthing_other, another for smthing -- nested parent like ../../
        }
        fn private_fn(){

        }
        pub struct NewStruct{

        }
        pub enum NewEnum{
            V1
        }
    }
}
fn abt_path(){
    // there are relative paths starts from current module like using self, super or any identifier
    // and there is absolute path that starts from crate:: which is the crate root(either lib or main)
    // Paths are followed by ::

    crate::smthing::smthing_other::print_path_type("absolute"); // absolute path

    crate::smthing::smthing_other::print_path_type("relative"); // relative path

    pub use crate::smthing::smthing_other; // creating shortcut, now the mod smthing_other can be called like other vars in the current scope. | pub means we reexporting it for external program to call it instead of our inner long module path
    use crate::smthing::smthing_other as smthing_other_other;
    smthing_other::print_path_type("use shortcut with");
    // smthing_other::private_fn(); // private fn, use respects that
    // IDIOMATIC WAY TO USE 'use' in rust:
    // for function: with parent as shown above
    // for data types like enum or struct, full path like(except if they multiple var has same name so with parent..):
    use smthing_other::{ NewStruct, NewEnum }; // importing multiple things from same path.
    let s = NewStruct{};
    let e = NewEnum::V1;

    // pub use smthing_other;
    // all items are private to parent by default but childs can see everything of ancestor...
}

fn public_private_data(){
    // structs
    mod purchase{
        #[derive(Debug)]
        pub struct Purchase { // the struct is public not its content
            pub item: String,
            price: u32,
            discount: u32,
            eta: String, // using for now only
            pub product_category: Category
        }
        fn get_price(item_name:&str) -> u32 {
            1000
        }
        fn get_sessional_discount() -> u32 {
            18
        }
        impl Purchase{
            pub fn make_purchase(item_name: &str) -> Purchase {
                Purchase{
                    item: item_name.to_string(),
                    price: get_price(item_name),
                    discount: get_sessional_discount(),
                    eta: String::from("Tomorrow"),
                    product_category: Category::Non_Tech
                }
            }
        }

        #[derive(Debug)]
        pub enum Category{ // pub in enum makes its all variants pub too
            Tech,
            Non_Tech
        }
    }
    let mut purchase = purchase::Purchase::make_purchase("Blahaj");
    purchase.item = String::from("Potato");
    // purchase.price = 10; // can't as its private
    purchase.product_category = purchase::Category::Tech;
    println!("{:?}", purchase); // private fields can be viewed but cannot be written
}
