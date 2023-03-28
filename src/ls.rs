use crate::data_structs::{Reservation, Vehicle, Zone};
extern crate rand;
use rand::{Rng, random};
use std::{fs::File, io::Write, time::Instant, process::exit};

pub struct LocalSearch {
    pub reservations: Vec<Reservation>,
    pub zones: Vec<Zone>,
    pub vehicle: Vec<Vehicle>,

    pub veh_to_res: Vec<Vec<i32>>, // given vehicle, get list of reservations that is assigned to it
    pub veh_to_zon: Vec<i32>,      // given vehicle, get zone that is assigned
    pub unassigned: Vec<i32>,

    pub best_cost: i32,
    pub best_veh_to_res: Vec<Vec<i32>>,
    pub best_veh_to_zon: Vec<i32>,
    pub best_unassigned: Vec<i32>,

    pub local_cost: i32,
    pub local_veh_to_res: Vec<Vec<i32>>,
    pub local_veh_to_zon: Vec<i32>,
    pub local_unassigned: Vec<i32>,
}

impl LocalSearch {
    fn does_interfere(&self, res1: usize, res2: usize) -> bool {
        let res_1: &Reservation = &self.reservations[res1];
        let res_2: &Reservation = &self.reservations[res2];

        if res_1.day != res_2.day {
            return false;
        }

        let start_1 = res_1.start;
        let end_1 = start_1 + res_1.restime;
        let start_2 = res_2.start;
        let end_2 = start_2 + res_2.restime;

        if (start_1 <= start_2) && (start_2 <= end_1) {
            return true;
        } else if (start_1 <= end_2) && (end_2 <= end_1) {
            return true;
        } else if (start_1 <= end_2) && (start_2 <= end_1) {
            return true;
        }
        return false;
    }

    fn does_list_interfere(&self, res_1: usize, res_list: &Vec<i32>) -> bool {
        for res_2 in res_list {
            if self.does_interfere(res_1, *res_2 as usize) {
                return true;
            }
        }
        return false;
    }

    fn set_vehicle_if_not_interfere(
        &mut self,
        res_1: usize,
        veh: usize,
    ) -> bool {
        if !self.does_list_interfere(res_1, &self.veh_to_res[veh]) {
            if self.unassigned.contains(&(res_1 as i32)) {
                self.veh_to_res[veh].push(res_1 as i32);
                self.assign_veh_to_res(veh as i32, res_1 as i32);
                return true;
            }
        }
        return false;
    }

    fn assign_veh_to_res(&mut self, veh_id: i32, res_id: i32) {
        self.veh_to_res[veh_id as usize].push(res_id);

        let index = self.unassigned.iter().position(|x| *x == res_id).unwrap();
        self.unassigned.remove(index);
    }

    fn assign_zon_to_veh(&mut self, veh_id: i32, zon_id: i32) {
        self.veh_to_zon[veh_id as usize] = zon_id;
    }

    fn vehicle_possible_own(&self, veh_id: usize, res_id: usize) -> bool {
        return self.reservations[res_id].zone == self.veh_to_zon[veh_id]
            && self.unassigned.contains(&(res_id as i32))
            && self.reservations[res_id]
                .possible_vehicles
                .contains(&(veh_id as i32));
    }

    fn vehicle_possible_neighbour(&self, veh_id: usize, res_id: usize) -> bool {
        return self.zones[self.veh_to_zon[veh_id] as usize]
            .neighbours
            .contains(&self.reservations[res_id].zone)
            && self.unassigned.contains(&(res_id as i32))
            && self.reservations[res_id]
                .possible_vehicles
                .contains(&(veh_id as i32));
    }

    pub fn new(res: Vec<Reservation>, zon: Vec<Zone>, veh: Vec<Vehicle>) -> LocalSearch {
        let veh_to_zon: Vec<i32> = vec![-1; veh.len()];
        let mut veh_to_res: Vec<Vec<i32>> = vec![];

        let best_veh_to_zon: Vec<i32> = vec![-1; veh.len()];
        let mut best_veh_to_res: Vec<Vec<i32>> = vec![];

        let local_veh_to_zon: Vec<i32> = vec![-1; veh.len()];
        let mut local_veh_to_res: Vec<Vec<i32>> = vec![];

        let mut unassigned: Vec<i32> = vec![];
        let mut best_unassigned: Vec<i32> = vec![];
        let mut local_unassigned: Vec<i32> = vec![];
        for _ in &veh {
            veh_to_res.push(vec![]);
            best_veh_to_res.push(vec![]);
            local_veh_to_res.push(vec![]);
        }
        for i in 0..res.len() {
            unassigned.push(i as i32);
            best_unassigned.push(i as i32);
            local_unassigned.push(i as i32);
        }
        LocalSearch {
            reservations: res,
            zones: zon,
            vehicle: veh,
            veh_to_res,
            veh_to_zon,
            unassigned,
            best_cost: 1000000,
            best_veh_to_res,
            best_veh_to_zon,
            best_unassigned,
            local_cost: 1000000,
            local_veh_to_res,
            local_veh_to_zon,
            local_unassigned,
        }
    }

    fn commit(&mut self)
    {
        self.local_veh_to_res = self.veh_to_res.clone();
        self.local_veh_to_zon = self.veh_to_zon.clone();
        self.local_unassigned = self.unassigned.clone();
        self.local_cost = self.calculate_full_cost();
    }

    fn restore(&mut self)
    {
        self.veh_to_res = self.local_veh_to_res.clone();
        self.veh_to_zon = self.local_veh_to_zon.clone();
        self.unassigned = self.local_unassigned.clone();
    }

    pub fn run(&mut self, time: i32) {
        self.initialise();

        let mut threshold = 0;
        let mut age = 1;

        self.commit();

        self.optimise();

        println!("cost_begin: {} and is: {}", self.best_cost, self.check_all());

        let start_time = Instant::now();

        let mut i = 0;

        while start_time.elapsed().as_secs() < time as u64 {
            i += 1;
            let mut better = false;
            let vehicle_id = rand::thread_rng().gen_range(0..self.vehicle.len());
            let neighbours = self.zones[self.veh_to_zon[vehicle_id]as usize].neighbours.clone();

            // for zone_id in neighbours {
            for zone_id in 0..self.zones.len() {
                self.car_to_zone(vehicle_id as i32, self.zones[zone_id as usize].id);
                let cost = self.calculate_full_cost();

                if self.check_all() && cost < self.best_cost  + threshold {
                    self.commit();
                    println!("\n    new self.best best found!");
                    better = true;
                } else {
                    self.restore();
                }
            }

            let cost = self.calculate_full_cost();
            if  better && self.check_all() {
                println!("\nnew best cost found! {cost}; age: {age}");

                self.commit();

                if self.local_cost < self.best_cost
                {
                    self.best_cost = self.local_cost;
                    self.best_unassigned = self.local_unassigned.clone();
                    self.best_veh_to_res = self.local_veh_to_res.clone();
                    self.best_veh_to_zon = self.local_veh_to_zon.clone();
                }

                age = 1;

            } else {
                age += 1;
                print!("\rage: {age}");
                let _ = std::io::stdout().flush();

                if age > 30 {
                    if self.optimise()
                    {
                        let cost = self.calculate_full_cost();
                        if self.check_all() && cost < self.local_cost {
                            let cost = self.calculate_full_cost();

                            self.local_cost = cost;

                            println!("\nnew best small cost: {cost}");
                            self.commit();
                            age = 1;

                            if self.local_cost < self.best_cost
                            {
                                self.best_cost = self.local_cost;
                                self.best_unassigned = self.local_unassigned.clone();
                                self.best_veh_to_res = self.local_veh_to_res.clone();
                                self.best_veh_to_zon = self.local_veh_to_zon.clone();
                            }

                        }
                    }
                    else {
                        self.restore();
                    }
                }
            }

            threshold = 0;

            // if age > 1000
            // {
            //     // reassign again because broken?
            //     let vehicle_id1 = rand::thread_rng().gen_range(0..self.vehicle.len());
            //     let mut vehicle_id2 = rand::thread_rng().gen_range(0..self.vehicle.len());
            //     while vehicle_id1 == vehicle_id2
            //     {
            //         vehicle_id2 = rand::thread_rng().gen_range(0..self.vehicle.len());
            //     }
            //     let zone_1 = self.veh_to_zon[vehicle_id1];
            //     let zone_2 = self.veh_to_zon[vehicle_id2];
            //     self.car_to_zone(vehicle_id1 as i32, self.zones[zone_2 as usize].id);
            //     self.car_to_zone(vehicle_id2 as i32, self.zones[zone_1 as usize].id);
            // }

            if age > 100 {
                threshold = 70;
            }
            // if age > 200 {
            //     threshold = 70;
            // }
            // if age > 300 {
            //     threshold = 100;
            // }
        }

        self.best_cost = self.calculate_full_cost();
        println!("\ncost_end: {} after {i} iterations", self.best_cost);
    }

    fn optimise(&mut self) -> bool {
        let mut changed = false;
        for i in 0..self.reservations.len() {
            self.small_operator(i as i32);
            let cost = self.calculate_full_cost();

            if cost < self.best_cost {
                self.commit();
                changed = true;
            } else {
                self.restore();
            }
        }
        changed
    }

    fn check_all(&self) -> bool
    {
        let amount_res = self.reservations.len();
        let mut res = 0;
        for reservations in &self.veh_to_res
        {
            res += reservations.len();
            for res_1 in reservations
            {
                for res_2 in reservations
                {
                    if res_1 == res_2
                    {
                        continue;
                    }
                    if self.does_interfere(*res_1 as usize, *res_2 as usize)
                    {
                        return false;
                    }
                }
            }
        }
        res += self.unassigned.len();
        if res != amount_res
        {
            return false;
        }
        true
    }

    fn small_operator(&mut self, res_id: i32) {
        // is zone of vehicle zone of reservation?
        let mut veh_id: usize = 0;
        let zon_res: i32 = self.reservations[res_id as usize].zone;
        for (veh, res) in self.veh_to_res.iter().enumerate() {
            if res.contains(&res_id) {
                veh_id = veh;
                break;
            }
        }
        if self.veh_to_zon[veh_id] == zon_res {
            return;
        }

        let mut assigned: Vec<i32> = vec![];

        // change vehicle from reservation to possible vehicle in own zone
        for veh in 0..self.veh_to_zon.len() {
            // check for vehicle in same zone as res_id
            if self.veh_to_zon[veh] == zon_res {
                if self.does_list_interfere(res_id as usize, &self.veh_to_res[veh]) {
                    if self.set_vehicle_if_not_interfere(res_id as usize, veh) {
                        if self.unassigned.contains(&res_id) {
                            assigned.push(res_id);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn car_to_zone(&mut self, veh_id: i32, zon_id: i32) {
        if zon_id == self.veh_to_zon[veh_id as usize] {
            return;
        }

        self.unassigned
            .append(&mut self.veh_to_res[veh_id as usize]);
        self.unassigned.dedup();

        self.veh_to_res[veh_id as usize] = vec![];

        self.veh_to_zon[veh_id as usize] = zon_id;

        // assign all reservations in own zone
        let unnassigned_copy = self.unassigned.clone();

        for res in unnassigned_copy
        {
            if self.vehicle_possible_own(veh_id as usize, res as usize)
            {
                self.set_vehicle_if_not_interfere(res as usize, veh_id as usize);
            }
        }

        let unnassigned_copy = self.unassigned.clone();

        // assign all neighbouring reservations to vehicle
        for res in unnassigned_copy
        {
            if self.vehicle_possible_neighbour(veh_id as usize, res as usize)
            {
                self.set_vehicle_if_not_interfere(res as usize, veh_id as usize);
            }
        }

        let unnassigned_copy = self.unassigned.clone();
        for res in unnassigned_copy
        {
            let zone = self.reservations[res as usize].zone;
            // loop through all vehicles and check if the zone of the reservation is the same as the zone of res
            for z_index in 0..self.veh_to_zon.len()
            {
                if self.veh_to_zon[z_index] == zone
                {
                    if self.vehicle_possible_own(z_index as usize, res as usize)
                    {
                        self.set_vehicle_if_not_interfere(res as usize, z_index);
                    }
                }
            }
        }

        for reservations in &mut self.veh_to_res
        {
            reservations.dedup();
        }

    }

    pub fn initialise(&mut self) {
        // sort by possible vehicles for reservation
        self.reservations
            .sort_by(|a, b| a.possible_vehicles.len().cmp(&b.possible_vehicles.len()));

        let mut used: Vec<i32> = vec![];

        let mut assigned: Vec<i32> = vec![];

        for res_it in 0..self.reservations.len() {
            for i in 0..self.reservations[res_it].possible_vehicles.len() {
                let veh_id = self.reservations[res_it].possible_vehicles[i];
                if !used.contains(&veh_id) {
                    used.push(veh_id as i32);
                    self.assign_zon_to_veh(veh_id, self.reservations[res_it].zone);
                    self.assign_veh_to_res(veh_id, self.reservations[res_it].id);
                    // self.set_vehicle_if_not_interfere(self.reservations[res_it].id as usize, veh_id as usize);
                    break;
                }
            }
        }

        self.reservations.sort_by(|a, b| a.id.cmp(&b.id));

        for veh_id in 0..self.vehicle.len() {
            for res_id in 0..self.reservations.len() {
                if self.vehicle_possible_own(veh_id, res_id) {
                    if self.set_vehicle_if_not_interfere(
                        self.reservations[res_id].id as usize,
                        self.vehicle[veh_id].id as usize,
                    ) {
                        assigned.push(res_id as i32);
                    }
                }
            }
        }

        for veh_id in 0..self.vehicle.len() {
            for res_id in 0..self.reservations.len() {
                if self.vehicle_possible_neighbour(veh_id, res_id) {
                    if self.set_vehicle_if_not_interfere(res_id, veh_id) {
                        assigned.push(res_id as i32);
                    }
                }
            }
        }

        for reservations in &mut self.veh_to_res
        {
            reservations.dedup();
        }
    }

    pub fn calculate_full_cost(&self) -> i32 {
        let mut sum = 0;
        for (veh_id, reservations) in self.veh_to_res.iter().enumerate() {
            for res in reservations {
                sum += self.calculate_cost(*res, veh_id);
            }
        }
        for res in &self.unassigned {
            sum += self.reservations[*res as usize].p1;
        }
        return sum;
    }

    fn calculate_cost(&self, res_id: i32, veh_id: usize) -> i32 {
        let zon_veh = self.veh_to_zon.get(veh_id).expect("expected zone");
        let zon_res = &self.reservations[res_id as usize].zone;

        if *zon_veh == *zon_res {
            return 0;
        }
        // } else if self.zones[*zon_veh as usize].neighbours.contains(zon_res) {
        return self.reservations[res_id as usize].p2;
        // }
    }

    pub fn write_output(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        file.write(format!("{}\n", self.best_cost).as_bytes())?;
        file.write(b"+Vehicle assignments\n")?;
        for (i, zone) in self.veh_to_zon.iter().enumerate() {
            file.write(format!("car{};z{}\n", i, zone).as_bytes())?;
        }

        file.write(b"+Assigned requests\n")?;
        for (i, veh) in self.veh_to_res.iter().enumerate() {
            for req in veh {
                file.write(format!("req{};car{}\n", req, i).as_bytes())?;
            }
        }

        file.write(b"+Unassigned requests\n")?;
        for res in &self.unassigned {
            file.write(format!("req{}\n", res).as_bytes())?;
        }
        Ok(())
    }
}
