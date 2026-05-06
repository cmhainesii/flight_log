use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ActualTimes {

    pub departure: Option<String>,
    pub arrival: Option<String>,
}


#[allow(dead_code)]
impl ActualTimes {
    pub fn print_actuals(&self) {
        match &self.departure {
            Some(departure_time) => {
                println!("Actual Departure: {departure_time}")
            }
            _ => { }
        }

        match &self.arrival {
            Some(arrival_time) => {
                println!("  Actual Arrival: {arrival_time}")
            }
            _ => { }
        }
    }
}