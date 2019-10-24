extern crate rand;

use std::io::prelude::*;
use rand::Rng;

#[derive(Debug)]
pub struct Mansion {
    pub rooms: Vec<String>,
}

#[derive(Debug)]
pub struct Suspect {
    pub fname: String,
    pub lname: String,
    pub occupation: String,
    pub resistance: i32,
}

impl Suspect {
    pub fn new(fname: String, lname: String, occupation: String) -> Suspect {
        let resist = rand::thread_rng().gen_range(1, 10);

        Suspect {fname: fname, lname: lname, occupation: occupation, resistance: resist}
    }
}

#[derive(Debug)]
pub struct MurderWeapon {
    pub weapons: Vec<String>,
}

#[derive(Debug)]
pub struct Guilty {
    person: String,
    place: String,
    method: String,
}

impl Guilty {
    pub fn create_guilty(perp: &Vec<Suspect>, place: Vec<String>, method: Vec<String>) -> Guilty {

        let get_suspect: usize = rand::thread_rng().gen_range(0, 6);
        let get_room: usize = rand::thread_rng().gen_range(0, 12);
        let get_weapon: usize = rand::thread_rng().gen_range(0, 8);

        let fname = perp[get_suspect].fname.to_string();
        let lname = perp[get_suspect].lname.to_string();

        let name: String = fname + " " + &lname;


        Guilty {person: name, place: place[get_room].to_string(), method: method[get_weapon].to_string()}
    }
}

pub fn create_mansion() -> Mansion {

        let mut rooms = Vec::new();

        rooms.push(String::from("Grand Foyer"));
        rooms.push(String::from("Cloak Room"));
        rooms.push(String::from("Gallery"));
        rooms.push(String::from("Dining Hall"));
        rooms.push(String::from("Library"));
        rooms.push(String::from("Ballroom"));
        rooms.push(String::from("Study"));
        rooms.push(String::from("Bathroom"));
        rooms.push(String::from("Drawing Room"));
        rooms.push(String::from("Garage"));
        rooms.push(String::from("Garden"));
        rooms.push(String::from("Shooting Range"));

        Mansion {rooms: rooms}
    }

pub fn create_suspects() -> Vec<Suspect> {
    let mut suspects: Vec<Suspect> = Vec::new();
    let mut count: i32 = 0;
    let mut person: Suspect;

    let fnames = vec![
        String::from("Andy"),
        String::from("Elizabeth"),
        String::from("David"),
        String::from("Mary"),
        String::from("Thomas"),
        String::from("Brandy"),
        String::from("Mathew"),
        String::from("Dorothy"),
        String::from("Creflo"),
        String::from("Amber"),
    ];

    let mut get_fname:usize;

    let lnames = vec![
        String::from("Teal"),
        String::from("Peacock"),
        String::from("Violet"),
        String::from("Cyan"),
        String::from("Grape"),
        String::from("Ketschup"),
        String::from("Pumpkins"),
        String::from("Dollar"),
        String::from("Mustard"),
        String::from("Blanc"),
    ];

    let mut get_lname:usize;

    let occups = vec![
        String::from("Factory Owner"),
        String::from("Televangelist"),
        String::from("Economics Professor"),
        String::from("High School Teacher"),
        String::from("Midwife"),
        String::from("Doctor"),
        String::from("Army Major"),
        String::from("Car Salesman"),
        String::from("Astronomer"),
        String::from("Psychic"),
    ];

    let mut get_occup:usize;

    let mut temp_fname: Vec<usize> = Vec::new();
    let mut temp_lname: Vec<usize> = Vec::new();
    let mut temp_occup: Vec<usize> = Vec::new();
    let mut tfn: usize = 13;
    let mut tln: usize = 13;
    let mut toc: usize = 13;

    loop {
        if count <= 5 {
            loop {
                get_fname = rand::thread_rng().gen_range(0, 10);
                let i:usize = 0;
                for i in 0..temp_fname.len() {
                    if get_fname == temp_fname[i] {
                        tfn = temp_fname[i].clone();
                    } 
                }
                if get_fname != tfn {
                    break;
                }
            }

            loop {
                get_lname = rand::thread_rng().gen_range(0, 10);
                let j:usize = 0;
                for j in 0..temp_lname.len() {
                    if get_lname == temp_lname[j] {
                        tln = temp_lname[j].clone();
                    } 
                }
                if get_lname != tln {
                    break;
                }
            }

            loop {
                get_occup = rand::thread_rng().gen_range(0, 10);
                let x:usize = 0;
                for x in 0..temp_occup.len() {
                    if get_occup == temp_occup[x] {
                        toc = temp_occup[x].clone();
                    } 
                }
                if get_occup != toc {
                    break;
                }
            }

            temp_fname.push(get_fname.clone());
            temp_lname.push(get_lname.clone());
            temp_occup.push(get_occup.clone());

            person = Suspect::new(fnames[get_fname].to_string(), lnames[get_lname].to_string(), occups[get_occup].to_string());

            suspects.push(person);
        } else {
            break;
        }

        count = count + 1;

    }

    suspects
}

pub fn weapons_list() -> MurderWeapon {

    let mut weapons = Vec::new();

    weapons.push(String::from("Revolver"));
    weapons.push(String::from("Rifle"));
    weapons.push(String::from("Knife"));
    weapons.push(String::from("Sword"));
    weapons.push(String::from("Poker"));
    weapons.push(String::from("Statue"));
    weapons.push(String::from("Rope"));
    weapons.push(String::from("pipe"));

    MurderWeapon {weapons: weapons}
}

pub fn roll_dice() -> i32 {
    let die1 = rand::thread_rng().gen_range(1, 7);
    let die2 = rand::thread_rng().gen_range(1, 7);
    let total: i32 = die1 + die2;

    println!("{} {}", die1, die2);

    total
}

pub fn show_weapons(weapon: &MurderWeapon) {
    println!("{:?}", weapon);
}

pub fn show_suspects(everyone: &Vec<Suspect>) {
    let i:usize = 0;
    for i in 0..everyone.len() {
        println!("{:?}", everyone[i]);
    }
    //println!("{:?}", everyone);
}

pub fn show_mansion(mansion: &Mansion) {
    println!("{:?}", mansion);
}

pub fn show_guilty(perp: &Guilty) {
    println!("{:?}", perp);
}

