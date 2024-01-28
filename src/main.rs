mod player_structs;
use player_structs::{Player, Status, Timestamps};

use reqwest;
use std::{thread, time::Duration};
use chrono::{NaiveDateTime};


fn main() {

    //list of all players that should be tracked..
    let players = vec!["Stolqe", "Sultan_Of_Aliens", "LukiSkradacz", "XY_Boy"];
    let mut i = 1; //times requested

 
    //for and while loop and request emc api
    loop {

        println!("Requesting player data...\n");

        //iterate over the players in the vector
        for n in 0..players.len() {
            let response = match reqwest::blocking::get("https://api.earthmc.net/v2/aurora/residents/".to_owned() + players[n]) {
                Ok(response) => response.text().unwrap(),
                Err(err) => panic!("Error {}", err)
            };

            //parse json and store other vars
            let json: Player = serde_json::from_str(&response).expect("json is invalid, are you sure you set the correct user?");
            let online: bool = json.status.isOnline;
            let last_online = NaiveDateTime::from_timestamp(json.timestamps.lastOnline / 1000, 0);

            //print if they are online
            if online == true {
                println!("{} is online.. beware..", players[n]);
            } else {
                println!("{} is not online.. (last online: {})", players[n], last_online);
            }
        }

        println!("\nTimes requested: {}", i);
        thread::sleep(Duration::from_secs(300)); //sleep for 5 mins
        i += 1;
        clearscreen::clear().unwrap();
    }
}
