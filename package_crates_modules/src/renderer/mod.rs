mod fps;

#[derive(Debug)]
pub enum Quality{
    Low,
    Fine,
    High
}

#[derive(Debug)]
pub struct Renderer{
    pub fps: u32,
    pub quality: Quality 
}

fn very_humble_task(){
    // doing malicious activity here
    // scary stuff, not for user..
}

pub fn print_smthing() {
    very_humble_task();
    println!("What if i say smthing here?!!");
}

