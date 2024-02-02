use std::io;
use reqwest;
use chrono::NaiveDateTime;

use crate::structs::{Town};
use crate::pause::pause;

pub fn lookup() {
    let mut town_name = String::new();

    println!("Enter town name: ");
    io::stdin().read_line(&mut town_name).unwrap();

    println!("\nRequsting town data...\n");

    //request data
    let response = match reqwest::blocking::get("https://api.earthmc.net/v2/aurora/towns/".to_owned() + &town_name) {
        Ok(response) => response.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };

    if response == "Invalid API route :(" {
        println!("Invalid nation name.\n");
        pause();
        std::process::exit(0);
    }

    //parse json and store vars, maybe this is too overkill...
    //i love horrible code
    let json: Town = serde_json::from_str(&response).unwrap();
    let x_coord: f32 = json.coordinates.spawn.x;
    let y_coord: f32 = json.coordinates.spawn.y;
    let z_coord: f32 = json.coordinates.spawn.z;
    let name: String = json.name;
    let mayor: String = json.mayor;
    let balance: f32 = json.stats.balance;
    let registered = NaiveDateTime::from_timestamp_opt(json.timestamps.registered / 1000, 0);
    let nation = json.nation;
    let is_public: bool = json.status.isPublic;
    let is_open: bool = json.status.isOpen;
    let is_ruined: bool = json.status.isRuined;


    //print data
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("Town Name: {}", name);
    println!("Mayor: {}", mayor);
    println!("Registered: {}", registered.unwrap().format("%d/%m/%Y %H:%M:%S"));
    println!("In Nation: {}", nation);
    println!("Gold in bank: {}", balance);
    if is_public { println!("Public: Yes"); } else { println!("Public: No"); };
    if is_open { println!("Open: Yes"); } else { println!("Open: No"); };
    if is_ruined { println!("Ruined: Yes"); } else { println!("Ruined: No"); };
    println!("X: {}", x_coord);
    println!("Y: {}", y_coord);
    println!("Z: {}", z_coord);
    println!("https://earthmc.net/map/aurora/?worldname=earth&mapname=flat&zoom=5&x={}&z={}", x_coord as i32, z_coord as i32);
    println!(":)");
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");

    pause();
    std::process::exit(0);
}
