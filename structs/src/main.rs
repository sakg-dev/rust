use std::io;
use std::io::Write;
// use std::thread;
use rand::Rng;

fn main(){
    define_and_instantiate();
}

fn prompt(message: &str, var_ref: &mut String){
    print!("{}", message);
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(var_ref).unwrap();
}

fn define_and_instantiate(){
    struct Player{
        name: String,
        health: u32,
        mode: String, // peaceful or combat
    }

    struct Coords(i32, i32, i32);

    struct Invincible;

    let _god_player = Invincible; // not sure its uses aaa

    let origin = Coords(0,0,0);
    let Coords(x, y, z) = origin;

    let mut name:String = String::new();
    prompt("Enter your name: ", &mut name);
    
    let mut player = Player {
        name: name.trim().to_string(), // i am not sure if this is good TODO
        health: 100,
        mode: String::from("peaceful")
    };
    
    println!("Player spawned at {}:{}:{}", x,y,z);

    let mut action_ipt:String = String::new();
    loop {
        action_ipt.clear();
        prompt("Action: ", &mut action_ipt);
        let action = &action_ipt.trim();
        println!("Your action is '{action}'");
        if action == &"help" {
            println!("\t- help: see all actions available\n\t- quit: exit the program\n\t- fight: fight with our bot to gain heart!\n\t- info: all info about the player");
        } else if action == &"quit" {
            break;
        } else if action == &"fight" {
            handle_fight(&mut player);
        } else if action == &"info" {
            println!("\n\tname: {}\n\thealth: {}\n\tmode: {}\n", player.name, player.health, player.mode)
        } else {
            println!("Uknown command, type 'help' to see available commands");
        }
       
    }

    fn handle_fight(player: &mut Player) {
        println!("lets fight {}!!", player.name);
        player.mode.clear();
        player.mode.push_str("combat");
        // println!("'{}'", player.mode);
        println!("Its a complete text based fight. Rules:\n\t- No healing until u either loses or win\n\t- type 'h' to hit, the faster u type and send, the more cps u get\n\t- if u win u get 1 parmanent heart(20 health), or if u lose, u lose 1 heart");
        let mut decision:String = String::new();
        prompt("Enter to start fight or write 'back' to go back to main menu: ", &mut decision);
        if !decision.find("back").is_none(){
            return;
        }

        // thread::spawn(move || {
        //    println!("New thread started!");
        //    println!("{}", player.name);
        // });
        //
        // Rn not sure how to spawn a new thread and communicate with primary thread, once learn
        // ill cmplt it TODO

        // loop {
            // println!("lets fightttt");
           // break; 
        // }
        let num = rand::thread_rng().gen_range(0..2);
        if num == 1 {
            player.health += 20;
            println!("You won the fight! Now your health is {}", player.health);
        } else {
            player.health -= 20;
            println!("You lost, your health is: {}", player.health);
        }
        player.mode.clear();
        player.mode.push_str("peaceful");
    }
}
