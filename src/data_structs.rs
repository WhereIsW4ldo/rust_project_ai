#[derive(Debug)]
pub struct Zone
{
    pub id: i32,
    pub neighbours: Vec<i32>
}

#[derive(Debug)]
pub struct Vehicle
{
    pub id: i32,
    pub zone: Option<i32>
}

#[derive(Debug)]
pub struct Reservation
{
    pub id: i32,
    pub zone: i32,
    pub day: i32,
    pub start: i32,
    pub restime: i32,
    pub possible_vehicles: Vec<i32>,
    pub p1: i32,
    pub p2: i32,
    pub vehicle: Option<i32>
}