#[derive(Debug)]
pub struct Zone
{
    pub id: i32,
    pub neighbours: Vec<i32>
}

#[derive(Debug)]
pub struct Vehicle<'a>
{
    pub id: i32,
    pub zone: &'a Option<Zone>
}

#[derive(Debug)]
pub struct Reservation<'a>
{
    pub id: i32,
    pub zone: &'a Option<Zone>,
    pub day: i32,
    pub start: i32,
    pub restime: i32,
    pub possible_vehicles: Vec<i32>,
    pub p1: i32,
    pub p2: i32,
    pub vehicle: &'a Option<Vehicle<'a>>
}