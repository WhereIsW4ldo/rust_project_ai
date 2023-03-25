use crate::data_structs::{Reservation, Zone, Vehicle};

pub struct LocalSearch<'a>
{
    pub reservations: Vec<Reservation<'a>>,
    pub zones: Vec<Zone>,
    pub vehicle: Vec<Vehicle<'a>>
}

impl LocalSearch<'_> 
{
    pub fn new<'a>(res: Vec<Reservation<'a>>, zon: Vec<Zone>, veh: Vec<Vehicle<'a>>) -> LocalSearch<'a>
    {
        LocalSearch { reservations: res, zones: zon, vehicle: veh }
    }
    
}