use atropat::lookup::{lookup};
use atropat::structs::{Player};

use reqwest;
use std::{thread, time::Duration};
use chrono::NaiveDateTime;
use std::io;


fn main() {
    let mut decision = String::new();

    //ask user if they want to do lookup or players
    println!("Enter 0 for Town Lookup or 1 for keeping an eye on players: ");
    io::stdin().read_line(&mut decision).unwrap();

    if decision.trim() == "0" {
        lookup();
    } else if decision.trim() == "1" {
        print!("\n");
        //do nothing and let program continue to loop
    } else {
        println!("Your option was invalid.");
    thread::sleep(Duration::from_secs(3));
        std::process::exit(1);
    }


    //players part of program, too lazy to turn into function lool
    //list of all players that should be tracked..
    let players = vec!["Stolqe", "Sultan_Of_Aliens", "LukiSkradacz", "XY_Boy"];

    let mut i = 1; //times requested

    //for and while loop and request emc api
    loop {

        println!("Requesting player data...\n");

        //iterate over the players in the vector
        for n in 0..players.len() {
            let response = match reqwest::blocking::get("https://api.earthmc.net/v2/aurora/residents/".to_owned() + players[n] + "/") {
                Ok(response) => response.text().unwrap(),
                Err(err) => panic!("Error {}", err)
            };

            //parse json and store other variables related to the player
            let json: Player = serde_json::from_str(&response).expect("json is invalid, are you sure you set the correct user?");
            let online: bool = json.status.isOnline;
            let last_online = NaiveDateTime::from_timestamp_opt(json.timestamps.lastOnline / 1000, 0);
            let balance: f32 = json.stats.balance;

            //print information
            if online {
                println!("{} is online.. beware.. (Gold: {})", players[n], balance);
            } else {
                println!("{} is not online.. (last online: {}) (Gold: {})", players[n], last_online.unwrap().format("%d/%m/%Y %H:%M:%S"), balance);
            }
        }

        println!("\nTimes requested: {}", i);
        thread::sleep(Duration::from_secs(300)); //sleep for 5 mins
        i += 1;
        clearscreen::clear().unwrap();
    }
}
