use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Aircraft;
use crate::Airline;


#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub id: Uuid,
    pub planned_departure_time: String,
    pub planned_arrival_time: String,
    pub flight_number: u16,
    pub airline: Airline,
    pub cruise_altitude: u32,
    pub departure_airport: String,
    pub arrival_airport: String,
    pub distance_nm: u32,
    pub route: String,
    pub aircraft: Aircraft,
    pub number_passengers: u32,
    pub zero_fuel_weight: f64,
    pub remarks: String,

}



impl LogEntry {
    pub fn get_load_percent(&self) -> f64 {
        self.zero_fuel_weight / self.aircraft.mzfw() * 100.0
    }

    pub fn get_psx_percent(&self) -> f64 {
        self.number_passengers as f64 / self.aircraft.mpsx() as f64 * 100.0
    }
}