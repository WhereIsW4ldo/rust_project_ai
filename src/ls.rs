use crate::data_structs::{Reservation, Vehicle, Zone};
extern crate rand;
use rand::Rng;
use std::{fs::File, io::Write, time::Instant};

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
        rem_unassigned: bool,
    ) -> bool {
        if !self.does_list_interfere(res_1, &self.veh_to_res[veh]) {
            if self.unassigned.contains(&(res_1 as i32)) {
                self.veh_to_res[veh].push(res_1 as i32);
                self.assign_veh_to_res(veh as i32, res_1 as i32, rem_unassigned);
                return true;
            }
        }
        return false;
    }

    fn assign_veh_to_res(&mut self, veh_id: i32, res_id: i32, rem_unassigned: bool) {
        self.veh_to_res[veh_id as usize].push(res_id);

        if rem_unassigned {
            let index = self.unassigned.iter().position(|x| *x == res_id).unwrap();
            self.unassigned.remove(index);
        }
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
        let mut unassigned: Vec<i32> = vec![];
        let mut best_unassigned: Vec<i32> = vec![];
        for _ in &veh {
            veh_to_res.push(vec![]);
            best_veh_to_res.push(vec![]);
        }
        for i in 0..res.len() {
            unassigned.push(i as i32);
            best_unassigned.push(i as i32);
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
        }
    }

    pub fn run(&mut self, time: i32) {
        self.initialise();

        let mut threshold = 0;
        let mut age = 1;

        self.best_veh_to_res = self.veh_to_res.clone();
        self.best_veh_to_zon = self.veh_to_zon.clone();
        self.best_unassigned = self.unassigned.clone();
        self.best_cost = self.calculate_full_cost();

        self.optimise();

        println!("cost_begin: {}", self.best_cost);

        let start_time = Instant::now();

        let mut i = 0;

        while start_time.elapsed().as_secs() < time as u64 {
            i += 1;
            let vehicle_id = rand::thread_rng().gen_range(0..self.vehicle.len());
            let mut local_best_cost = self.best_cost;
            let mut local_veh_to_res = self.veh_to_res.clone();
            let mut local_veh_to_zon = self.veh_to_zon.clone();
            let mut local_unassigned = self.unassigned.clone();

            for i in 0..self.zones.len() {
                self.car_to_zone(vehicle_id as i32, i as i32);
                let cost = self.calculate_full_cost();

                if cost < local_best_cost {
                    local_best_cost = cost;
                    local_veh_to_res = self.veh_to_res.clone();
                    local_veh_to_zon = self.veh_to_zon.clone();
                    local_unassigned = self.unassigned.clone();
                    println!("\n    new local best found!");
                } else {
                    self.veh_to_res = local_veh_to_res.clone();
                    self.veh_to_zon = local_veh_to_zon.clone();
                    self.unassigned = local_unassigned.clone();
                }
            }
            self.veh_to_res = local_veh_to_res.clone();
            self.veh_to_zon = local_veh_to_zon.clone();
            self.unassigned = local_unassigned.clone();

            let cost = self.calculate_full_cost();
            if cost != self.best_cost && cost < self.best_cost + threshold {
                println!("\nnew best cost found! {cost}; age: {age}");

                self.best_veh_to_res = self.veh_to_res.clone();
                self.best_veh_to_zon = self.veh_to_zon.clone();
                self.best_unassigned = self.unassigned.clone();
                self.best_cost = cost;

                age = 1;
            } else {
                age += 1;
                print!("\rage: {age}");
                let _ = std::io::stdout().flush();
                self.veh_to_res = self.best_veh_to_res.clone();
                self.veh_to_zon = self.best_veh_to_zon.clone();
                self.unassigned = self.best_unassigned.clone();

                if age > 30 {
                    if self.optimise() {
                        let cost = self.calculate_full_cost();

                        println!("\nnew best small cost: {cost}");
                        self.best_veh_to_res = self.veh_to_res.clone();
                        self.best_veh_to_zon = self.veh_to_zon.clone();
                        self.best_unassigned = self.unassigned.clone();
                        age = 1;
                    }
                }
            }

            threshold = 0;

            // if age > 10 {
            //     threshold = 50;
            // }
            // if age > 20 {
            //     threshold = 70;
            // }
            // if age > 30 {
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
                self.best_cost = cost;
                self.best_veh_to_res = self.veh_to_res.clone();
                self.best_veh_to_zon = self.veh_to_zon.clone();
                self.best_unassigned = self.unassigned.clone();
                changed = true;
                println!("optimised!");
            } else {
                self.veh_to_res = self.best_veh_to_res.clone();
                self.veh_to_zon = self.best_veh_to_zon.clone();
                self.unassigned = self.best_unassigned.clone();
            }
        }
        changed
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
                    if self.set_vehicle_if_not_interfere(res_id as usize, veh, true) {
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

        self.veh_to_res[veh_id as usize] = vec![];

        self.veh_to_zon[veh_id as usize] = zon_id;

        let mut assigned: Vec<i32> = vec![];

        // assign all reservations in own zone
        for res in 0..self.unassigned.len() {
            if self.vehicle_possible_own(veh_id as usize, self.unassigned[res] as usize) {
                if self.set_vehicle_if_not_interfere(
                    self.unassigned[res] as usize,
                    veh_id as usize,
                    false,
                ) {
                    assigned.push(self.unassigned[res]);
                }
            }
        }

        for a in assigned {
            let index = self.unassigned.iter().position(|x| *x == a).unwrap();
            self.unassigned.remove(index);
        }

        assigned = vec![];

        // assign all reservations in neighbouring zone
        for res in 0..self.unassigned.len() {
            if self.vehicle_possible_neighbour(veh_id as usize, self.unassigned[res] as usize) {
                if self.set_vehicle_if_not_interfere(
                    self.unassigned[res] as usize,
                    veh_id as usize,
                    false,
                ) {
                    assigned.push(self.unassigned[res]);
                }
            }
        }

        for a in &assigned {
            let index = self.unassigned.iter().position(|x| *x == *a).unwrap();
            self.unassigned.remove(index);
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
                    self.assign_veh_to_res(veh_id, self.reservations[res_it].id, true);
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
                        true,
                    ) {
                        assigned.push(res_id as i32);
                    }
                }
            }
        }

        for veh_id in 0..self.vehicle.len() {
            for res_id in 0..self.reservations.len() {
                if self.vehicle_possible_neighbour(veh_id, res_id) {
                    if self.set_vehicle_if_not_interfere(res_id, veh_id, true) {
                        assigned.push(res_id as i32);
                    }
                }
            }
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
