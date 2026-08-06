use std::io;
use std::io::Write;
// use std::thread;
use rand::Rng;

fn main(){
    // define_and_instantiate();
    // example_and_dbg();
    methods();
}

fn methods(){
    struct Platforms {
        windows: bool,
        linux: bool,
        mac: bool
    }

    impl Platforms {
        fn gen_str(&self) -> String {
            let mut oses = String::from("");
            if self.windows{ oses.push_str("windows, ") }
            if self.linux{ oses.push_str("linux, ") }
            if self.mac{ oses.push_str("macs, ") }

            // println!("'{}'", &oses[(oses.len()-2)..]); // shows ', '
            oses = oses[..(oses.len()-2)].to_string();
            return oses;
        }
    }

    struct Game {
        name: String,
        dimension: u32,
        platforms: Platforms,
        downloads: u32, 
        cost: f32
    }

    impl Game {
        fn new(name:&str, dimension: u32, platforms: Platforms, downloads: u32, cost: f32) -> Self{
            Game{
                name:name.to_string(),
                dimension,
                platforms,
                downloads,
                cost
            }
        }

        fn release(&self) {
            println!("Yo, finally the waiting is over bcz {} has been finally released under the cost of just ${}, and its a {}d game, compatible for {}. Buy soon as {} ppl have already bought it!", self.name, self.cost, self.dimension, self.platforms.gen_str(), self.downloads)
        }

        fn compare(&self, other_game: &Game, mode: &str) {
            if mode == "strong"{
                println!("Jst believe me u tiny gamer, i have played way more games than u and u would love playing {} over {}", self.name, other_game.name);
            } else{
                println!("{} has {} players and costs {} whereas {} has {} players and costs {}", other_game.name, other_game.downloads, other_game.cost, self.name, self.downloads, self.cost);
            }
        }
    }

    // let portal_2 = Game{
    //    name: String::from("Portal 2"),
    //    dimension: 3,
    //    platforms: Platforms {
    //        windows: true,
    //        linux: true,
    //        mac: false
    //    },
    //    downloads: 1585543,
    //    cost: 4.27
    //};

    //let portal_1 = Game {
    //    name: String::from("Portal 1"),
    //    dimension: 3,
    //    platforms: Platforms {
    //        windows: true,
    //        linux: false,
    //        mac: false
    //    },
    //    downloads: 985554,
    //    cost: 3.25
    // };


    let portal_1 = Game::new("Portal 1", 3, Platforms{windows:true,linux:false,mac:false}, 985554, 3.25);
    let portal_2 = Game::new("Portal 2", 3, Platforms{windows:true,linux:true,mac:false}, 1585543, 4.27);
    
    portal_1.release();
    portal_2.release();
    portal_2.compare(&portal_1, "strong");
}

fn example_and_dbg(){
    #[derive(Debug)]
    struct Rect{
        width: u32,
        height: u32
    }

    fn area(rect: Rect) -> u32{
        rect.width * rect.height
    }

    let rect1 = Rect{
        width: 15,
        height: 11
    };

    // println!("{}", area(rect1))
    // println!("{:?}", rect1)
    // println!("{:#?}", rect1);
    // let a = [1,2,3,4,5,6,7,8,9];
    // println!("{:#?}", a)

    let rect2 = Rect{
        width: dbg!(3*4),
        height: 15
    };

    dbg!(&rect2); // bcz it takes ownership but we dont wanna move ownership

}

fn define_and_instantiate(){
    fn prompt(message: &str, var_ref: &mut String){
        print!("{}", message);
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(var_ref).unwrap();
    }

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
