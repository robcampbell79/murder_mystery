use murder_mystery::Suspect;
use murder_mystery::create_suspects;
use murder_mystery::show_suspects;
use murder_mystery::Mansion;
use murder_mystery::create_mansion;
use murder_mystery::show_mansion;
use murder_mystery::MurderWeapon;
use murder_mystery::weapons_list;
use murder_mystery::Guilty;
use murder_mystery::show_guilty;
use murder_mystery::roll_dice;
use murder_mystery::create_witness;
use murder_mystery::show_card;
use std::process::Command;
use std::io;

fn main() {

    let mut play: String;
    let token: bool;
    let mut roll: String;
    let mut rollInvestigate: String;
    let mut rollInterrogate: String;
    let mut dice: i32;
    let mut room: &str;
    let mut choice: String;
    let mut moveToRoom: String;
    let mut didMove: bool = false;
    let mut personToQuestion: String;
    let mut turns: i32 = 30;

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

        let guilty = Guilty::create_guilty(&s, &m.rooms, &w.weapons);

        let wtn = create_witness(&s, &guilty);

        show_guilty(&guilty);

        room = &m.rooms[0];

        println!("Welcome to Owl Haven manor, Detective. The suspects are inside. Mr. D'Ceased was found in the Grand Foyer, but it is believed that he was moved from the original crime scene. The suspects are inside.");
        println!("");

        loop {
            //show_card(&s, &w.weapons);

            println!("Type roll and press enter to roll the dice that begins your turn.");

            roll = String::new();

            io::stdin().read_line(&mut roll).expect("Invalid input");

            match roll.trim() {
                "roll" => dice = roll_dice(),
                _ => continue,
            }

            //dice = roll_dice();

            while dice > 0 {
                println!("What would you like to do?");
                println!("0: Show card.");
                println!("1: Move to a room.");
                println!("2: Investigate room.");
                println!("3: interrogate a suspect.");
                println!("4: Reveal guilty.");
                println!("5: End turn.");
                println!("Dice: {}", dice);

                choice = String::new();

                io::stdin().read_line(&mut choice).expect("Invalid input.");

                if choice.trim() == "1" {
                    println!("You are in {}", room);
                    println!("What room would you like to move to?");
                    show_mansion(&m.rooms);

                    moveToRoom = String::new();

                    io::stdin().read_line(&mut moveToRoom).expect("Invalid input.");

                    let moveToRoom: i32 = match moveToRoom.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    let index = m.rooms.iter().position(|r| r == room).unwrap();

                    let mut walk = moveToRoom - index as i32;

                    if walk < 0 {
                        walk = walk * -1;
                    }

                    walk = walk * 3;

                    if dice >= walk {
                        room = &m.rooms[moveToRoom as usize];

                        dice = dice - walk;

                        println!("You are now in the {}", room);

                        didMove = true;
                    } else {
                        println!("You can't move.");
                    }

                    //println!("index {} / dice {}", index, dice);
                }
                else if choice.trim() == "2" {
                    println!("Crime scene {}", guilty.place);
                    if dice >= 2 {
                        dice = dice - 2;

                        println!("Type roll to see if you successfully conduct an investigation.");

                        rollInvestigate = String::new();

                        let hiddenEvidence = roll_dice();

                        let investigateRoom: i32;

                        io::stdin().read_line(&mut rollInvestigate).expect("Invalid input");

                        match rollInvestigate.trim() {
                            "roll" => investigateRoom = roll_dice(),
                            _ => investigateRoom = 0,
                        }

                        println!("Your roll {}", investigateRoom);

                        //let investigateRoom = roll_dice();

                        if investigateRoom > hiddenEvidence {
                            if room == guilty.place {
                                println!("You find a {} covered in blood and blood spatter on a far wall. This must be the room where the murder took place.", guilty.method);
                            } else {
                                println!("Nothing seems to be out of order here.");
                            }
                        } else {
                            println!("You were unable to conduct a complete investigation.");
                        }
                    } else {
                        println!("You don't have enough moves.");
                    }
                }
                else if choice.trim() == "3" {
                    if dice > 5 {

                        if didMove == true {
                            println!("Who would you like to interrogate?");

                            show_suspects(&s);

                            personToQuestion = String::new();

                            io::stdin().read_line(&mut personToQuestion).expect("Invalid input.");

                            let personToQuestion: usize = match personToQuestion.trim().parse() {
                                Ok(num) => num,
                                Err(_) => continue,
                            };

                            println!("You ask {} {} some very intense questions", &s[personToQuestion].fname, &s[personToQuestion].lname);

                            let resistance = &s[personToQuestion].resistance;

                            println!("Type roll to see if you get answers.");

                            rollInterrogate = String::new();

                            let ask: i32;

                            io::stdin().read_line(&mut rollInterrogate).expect("Invalid input");

                            match rollInterrogate.trim() {
                                "roll" => ask = roll_dice(),
                                _ => ask = 0,
                            }

                            println!("Your roll {}", ask);

                            //let ask: i32 = roll_dice();

                            if &ask > resistance {
                                if &s[personToQuestion].fname == &wtn {
                                    println!("Detective, I saw {} sneaking off.", guilty.person);
                                } else {
                                    println!("I didn't see anything, Detective");
                                }
                            } else {
                                println!("{} {} refused to cooperate.", &s[personToQuestion].fname, &s[personToQuestion].lname);
                            }

                            dice = dice - 5;
                            didMove = false;

                        } else {
                            println!("You have to move to another room first, Detective.");
                        }
                    } else {
                        println!("You don't have enough moves.");
                    }
                }
                else if choice.trim() == "4" {
                    println!("You win!");
                }
                else if choice.trim() == "5" {
                    dice = 0;
                }
                else if choice.trim() == "0" {
                    show_card(&s, &w.weapons, &m.rooms);
                }

                turns = turns - 1;

                if turns == 0 {
                    println!("You were murdered, you lose!");
                    break;
                }
            }

        }
    } else {
        std::process::exit(0);
    }

}
