use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Aircraft {
    B737,
    B77L,
    B77F,
    A20N,
    B738
}

impl Aircraft {
    pub fn icao(&self) -> &'static str {
        match self {
            Aircraft::A20N => "A20N",
            Aircraft::B737 => "B737",
            Aircraft::B738 => "B738",
            Aircraft::B77F => "B77F",
            Aircraft::B77L => "B77L"
        }
    }

    pub fn mzfw(&self) -> f64 {
        match self {
            Aircraft::A20N => 141_757.0,
            Aircraft::B737 => 121_700.0,
            Aircraft::B738 => 138_300.0,
            Aircraft::B77F => 547_000.0,
            Aircraft::B77L => 461_000.0,
        }
    }

    pub fn mpsx(&self) -> u32 {
        match self {
            Aircraft::A20N => 174,
            Aircraft::B737 => 132,
            Aircraft::B738 => 163,
            Aircraft::B77F => 4,
            Aircraft::B77L => 297,
        }
    }
}

impl fmt::Display for Aircraft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Aircraft::A20N => write!(f, "Airbus A320"),
            Aircraft::B737 => write!(f, "Boeing 737-700"),
            Aircraft::B738 => write!(f, "Boeing 737-800"),
            Aircraft::B77F => write!(f, "Boeing 777-F"),
            Aircraft::B77L => write!(f, "Boeing 777-200LR"),
        }
    }
}