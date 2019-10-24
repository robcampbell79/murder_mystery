use murder_mystery::Suspect;
use murder_mystery::create_suspects;
//use murder_mystery::show_suspects;
use murder_mystery::Mansion;
use murder_mystery::create_mansion;
use murder_mystery::show_mansion;
use murder_mystery::MurderWeapon;
use murder_mystery::weapons_list;
use murder_mystery::Guilty;
use murder_mystery::show_guilty;
use murder_mystery::roll_dice;
//use murder_mystery::show_weapons;
use murder_mystery::show_card;
use std::process::Command;
use std::io;

fn main() {

    let mut play: String;
    let token: bool;
    let mut roll: String;
    let mut dice: i32;
    let mut room: String;
    let mut choice: String;

    println!("Murder Mystery");

    println!("");
    println!("Do you want to play?");

    play = String::new();

    io::stdin().read_line(&mut play).expect("Invalid input.");

    match play.trim() {
        "Yes" | "Y" | "yes" | "y" | "YES" => token = true,
        _ => token = false,
    }

    if token == true {

        let s = create_suspects();

        let m = create_mansion();

        //show_mansion(&m);

        let w = weapons_list();

        let guilty = Guilty::create_guilty(&s, m.rooms, &w.weapons);

        show_guilty(&guilty);

        room = "Grand Foyer";

        println!("Welcome to Owl Haven manor, Detective. The suspects are inside. Mr. D'Ceased was found in the Grand Foyer, but it is believed that he was moved from the original crime scene. The suspects are inside.");
        println!("");

        loop {
            show_card(&s, &w.weapons);

            println!("Type roll and press enter to roll the dice that begins your turn.");

            dice = roll_dice();

            while dice > 0 {
                println!("What would you like to do?");
                println!("1: Move to a room.");
                println!("2: Investigate room.");
                println!("3: Interragate a suspect.");
                println!("4: Reveal guilty.");

                choice = String::new();

                io::stdin().read_line(&mut choice).expect("Invalid input.");

                if choice == "1" {
                    println!("You are in {}", room);
                }
            }

        }
    } else {
        std::process::exit(0);
    }

}
