use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, VariantNames};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumIter, VariantNames)]
pub enum Airline {
    DAL,
    AAL,
    UAL,
    JBU,
    FDX,
    SWA,
    UPS,
}

impl Airline {
    pub fn icao(&self) -> &'static str {
        match self {
            Airline::DAL => "DAL",
            Airline::AAL => "AAL",
            Airline::UAL => "UAL",
            Airline::JBU => "JBU",
            Airline::FDX => "FDX",
            Airline::SWA => "SWA",
            Airline::UPS => "UPS",
        }
    }
}

impl fmt::Display for Airline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Airline::DAL => write!(f, "Delta Airlines"),
            Airline::AAL => write!(f, "American Airlines"),
            Airline::FDX => write!(f, "FedEx"),
            Airline::JBU => write!(f, "Jetblue"),
            Airline::SWA => write!(f, "Southwest Airlines"),
            Airline::UAL => write!(f, "United Airlines"),
            Airline::UPS => write!(f, "UPS Air Cargo"),
        }
    }
}