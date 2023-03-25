use std::fs;

use crate::data_structs::{Reservation, Zone, Vehicle};

pub fn read_file(filepath: &str) -> (Vec<Reservation>, Vec<Zone>, Vec<Vehicle>)
{
    // (Vec<data_structs::Reservation>, Vec<data_structs::Zone>, Vec<data_structs::Vehicle>, Vec<Vec<bool>>)
    println!("Contents of file {}:", filepath);

    let lines = fs::read_to_string(filepath)
                    .expect("could not read file");

    let mut amount_requests = 0;
    let mut amount_zones = 0;
    let mut amount_vehicles = 0;
    let mut vec_reservations: Vec<Reservation> = Vec::new();
    let mut vec_zones: Vec<Zone> = Vec::new();
    let mut vec_vehicles: Vec<Vehicle> = Vec::new();
    
    for (i, line) in lines.split("\n").enumerate()
    {
        let contents = line.split(";").collect::<Vec<&str>>();

        if i == 0 // get amount of reservations
        {
            let mut string = String::from(contents[0].split(": ").collect::<Vec<&str>>()[1]);
            if string.ends_with('\r')
            {
                string.pop();
            }
            if string.ends_with('\n')
            {
                string.pop();
            }
            amount_requests = string.parse().unwrap();
            println!("amount_requests: {amount_requests}");
            continue;
        }

        if i <= amount_requests // read in all requests
        {
            let id: i32 = contents[0][3..].parse().unwrap();
            // let zone: i32 = contents[1][1..].parse().unwrap();
            let day: i32 = contents[2].parse().unwrap();
            let start: i32 = contents[3].parse().unwrap();
            let restime: i32 = contents[4].parse().unwrap();
            let pos_veh: Vec<&str> = contents[5].split(',').collect();
            let mut possible_vehicles: Vec<i32> = Vec::new();
            for pos in pos_veh
            {
                let veh = pos[3..].parse::<i32>().unwrap();
                possible_vehicles.push(veh);
            }
            let p1: i32 = contents[6].parse().unwrap();
            let mut string = String::from(contents[7]);
            if string.ends_with('\r')
            {
                string.pop();
            }
            if string.ends_with('\n')
            {
                string.pop();
            }
            let p2: i32 = string.parse().unwrap();
            let res: Reservation = Reservation { id, zone: &None, day, start, restime, possible_vehicles, p1, p2, vehicle: &None };

            vec_reservations.push(res);
            continue;
        }

        if i == amount_requests + 1 // get amount of zones
        {
            let mut string = String::from(contents[0].split(": ").collect::<Vec<&str>>()[1]);
            if string.ends_with('\r')
            {
                string.pop();
            }
            if string.ends_with('\n')
            {
                string.pop();
            }
            amount_zones = string.parse().unwrap();
            println!("amount_zones: {amount_zones}");
            continue;
        }

        if i <= amount_requests + amount_zones + 1 // reading zones
        {
            let id: i32 = contents[0][1..].parse().unwrap();
            let strings: Vec<&str> = contents[1].split(',').collect();
            let mut zones: Vec<i32> = Vec::new();
            for s in strings
            {
                zones.push(s[1..].parse::<i32>().unwrap());
            }
            vec_zones.push(Zone{ id, neighbours: zones });
            continue;
        }

        if i == amount_requests + amount_zones + 2 // reading amount of vehicles
        {
            let mut string = String::from(contents[0].split(": ").collect::<Vec<&str>>()[1]);
            if string.ends_with('\n')
            {
                string.pop();
            }
            if string.ends_with('\r')
            {
                string.pop();
            }
            amount_vehicles = string.parse().unwrap();
            println!("amount_vehicles: {amount_vehicles}");
            continue;
        }

        if i <= amount_requests + amount_zones + amount_vehicles + 2
        {
            vec_vehicles.push(Vehicle { id: contents[0][3..].parse().unwrap(), zone: &None });
            continue;
        }
    }
    return (vec_reservations, vec_zones, vec_vehicles);
}