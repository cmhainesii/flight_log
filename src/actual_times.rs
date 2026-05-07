use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ActualTimes {

    pub startup: Option<String>,
    pub taxi: Option<String>,
    pub takeoff: Option<String>,
    pub in_flight: Option<String>,
    pub landed: Option<String>,
    pub shutdown: Option<String>,
}


#[allow(dead_code)]
impl ActualTimes {
    pub fn print_actuals(&self) {
        match &self.startup {
            Some(startup) => {
                println!("Startup: {startup}")
            }
            _ => { }
        }

        match &self.taxi {
            Some(taxi) => {
                println!("Taxi: {taxi}")
            }
            _ => { }
        }

        match &self.takeoff {
            Some(takeoff) => {
                println!("Takeoff: {takeoff}");
            }
            _ => { }
        }

        match &self.in_flight {
            Some(in_flight) => {
                println!("In-Flight: {in_flight}");
            }
            _ => { }
        }

        match &self.landed {
            Some(landed) => {
                println!("Landed: {landed}");
            }
            _ => { }
        }

        match & self.shutdown {
            Some(shutdown) => {
                println!("Shutdown: {shutdown}");
            }
            _ => { }
        }

    }
}