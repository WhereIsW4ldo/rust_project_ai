use std::os::unix::ucred::impl_linux;

use crate::data_structs::{Reservation, Zone, Vehicle};

pub struct LocalSearch<'a>
{
    pub reservations: Vec<Reservation<'a>>,
    pub zones: Vec<Zone>,
    pub vehicle: Vec<Vehicle<'a>>
}

impl LocalSearch<'_> 
{
    pub fn new(res: Vec<Reservation>, zon: Vec<Zone>, veh: Vec<Vehicle>) -> LocalSearch
    {
        LocalSearch { reservations: res, zones: zon, vehicle: veh }
    }

}