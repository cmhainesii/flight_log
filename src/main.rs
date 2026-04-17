use core::fmt;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::vec;


use chrono::NaiveDateTime;
use inquire::CustomType;
use inquire::CustomUserError;
use inquire::Select;
use inquire::Text;
use inquire::validator::Validation;
use serde::Deserialize;
use serde::Serialize;
use thousands::Separable;
use uuid::Uuid;

const FILENAME: &str = "logbook.nbc";


#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    id: Uuid,
    planned_departure_time: String,
    planned_arrival_time: String,
    flight_number: u16,
    airline: Airline,
    cruise_altitude: u32,
    departure_airport: String,
    arrival_airport: String,
    distance_nm: u32,
    route: String,
    aircraft: Aircraft,
    number_passengers: u32,
    zero_fuel_weight: f64,
}

struct FleetData {
    // Key: ICAO String, Value Max Passengers
    aircraft_specs: HashMap<String, u16>,
}

impl FleetData {
    fn new() -> Self {
        let mut specs = HashMap::new();
        specs.insert("A20N", 174);
        specs.insert("B77L", 297);
        specs.insert("B77F", 4);
    }
}




fn format_altitude(altitude: u32) -> String {
    if altitude >= 18_000 {
        format!("FL{:03}", altitude / 100)
    } else {
        format!("{} ft", altitude.separate_with_commas())
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum Aircraft {
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


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum Airline {
    DAL,
    AAL,
    UAL,
    JBU,
    FDX,
    SWA,
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
        }
    }
}

fn build_log_entry() -> LogEntry {

    // Create a new flight plan via command line interface!
    let id = Uuid::new_v4();


    let datetime_validator = |input: &str| -> Result<Validation, CustomUserError> {
        match NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M") {
            Ok(_) => Ok(Validation::Valid),
            Err(_) => Ok(Validation::Invalid("Format must be YYYY-MM-DD HH:MM (e.g. 2026-04-20 19:00)".into())),
        }
    };

    let flight_number_validator = |input: &str| -> Result<Validation, CustomUserError> {
        match input.parse::<u32>() {
            Ok(num) if num >= 100 && num <= 9999 => Ok(Validation::Valid),
            _ => Ok(Validation::Invalid("Flight number must be 3 or 4 digits".into()))
        }
    };

    let altitude_validator = |input: &u32| -> Result<Validation, CustomUserError> {
        if *input >= 500 && *input <= 100000 {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Altitude must be between 500 and 100,000 ft.".into()))
        }
    };

    let airport_icao_validator = |input: &str| -> Result<Validation, CustomUserError> {
        if input.len() == 4 && input.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Airport ICAO must be exactly 4 alphabetic characters".into()))
        }
    };

    let not_empty_validator = |input: &str| -> Result<Validation, CustomUserError> {
        match input.len() {
            num if num > 0 => Ok(Validation::Valid),
            _ => Ok(Validation::Invalid("Route cannot be empty.".into())),
        }
    };

    let above_zero_validator = |input: &str| -> Result<Validation, CustomUserError> {
        match input.parse::<u32>() {
            Ok(num) if num > 0 => Ok(Validation::Valid),
            _ => Ok(Validation::Invalid("Value must be greater than zero.".into()))
        }
    };

    let planned_departure_time = Text::new("Planned departure time?")
        .with_placeholder("2026-04-09 14:30")
        .with_validator(datetime_validator)
        .prompt()
        .expect("Invalid departure time");

    println!("Planned departure time: {}", planned_departure_time);

    let planned_arrival_time = Text::new("Planned arrival time?")
        .with_placeholder("2025-04-20 18:00")
        .with_validator(datetime_validator)
        .prompt()
        .expect("Invalid arrival time.");

    let flight_number = Text::new("Flight Number?")
        .with_placeholder("2939")
        .with_validator(flight_number_validator)
        .prompt()
        .expect("Invalid flight number.")
        .parse::<u16>()
        .expect("Failed to parse input as a number.");

    let options = vec![
        Airline::DAL,
        Airline::AAL,
        Airline::FDX,
        Airline::JBU,
        Airline::SWA,
        Airline::UAL,
    ];

    

    let airline = Select::new("Airline?", options)
        .prompt()
        .expect("Invalid airline.");

    let cruise_altitude = CustomType::<u32>::new("Cruise Altitude (feet)?")
        .with_placeholder("39000")
        .with_validator(altitude_validator)
        .prompt()
        .expect("Invalid cruise altitude.");

    let departure_airport = Text::new("Departure airport ICAO?")
        .with_placeholder("KPHL")
        .with_validator(airport_icao_validator)
        .prompt()
        .expect("Invalid departure airport ICAO.")
        .to_uppercase();

    let arrival_airport = Text::new("Arrival airport ICAO?")
        .with_placeholder("KBOS")
        .with_validator(airport_icao_validator)
        .prompt()
        .expect("Invalid arrival airport ICAO.")
        .to_uppercase();

    let distance_nm = CustomType::<u32>::new("Distance? (nm)")
        .with_placeholder("203")
        .with_help_message("Enter planned route distance in NM")
        .with_error_message("Please enter a valid distance in NM")
        .prompt()
        .expect("Error parsing distance.");

    let route = Text::new("Route?")
        .with_placeholder("KPHL DCT DITCH DCT LUIGI DCT HNNAH DCT JFK ROBUC3 KBOS")
        .with_validator(not_empty_validator)
        .prompt()
        .expect("Invalid route.")
        .to_uppercase();

    
    let options = vec![
        Aircraft::B737,
        Aircraft::B738,
        Aircraft::B77F,
        Aircraft::B77L,
        Aircraft::A20N,
    ];

    let aircraft = Select::new("Aircraft?", options)
        .prompt()
        .expect("Invalid aircraft.");

    let number_passengers = Text::new("Passengers?")
        .with_placeholder("193")
        .with_validator(above_zero_validator)
        .prompt()
        .expect("Invalid number of passengers.")
        .parse::<u32>()
        .expect("Failed to parse input as a number");

    let zero_fuel_weight = Text::new("Expected ZFW?")
        .with_placeholder("125985")
        .with_validator(above_zero_validator)
        .prompt()
        .expect("Invalid ZFW.")
        .parse::<f64>()
        .expect("Failed to parse input as a number.");

    println!("    Assigned ID Number: {}", id);
    println!("Planned departure time: {}", planned_departure_time);
    println!("  Planned arrival time: {}", planned_arrival_time);
    println!("         Flight number: {}", flight_number);
    println!("               Airline: {}", airline);
    println!("          Airline ICAO: {}", airline.icao());
    println!("       Cruise Altitude: {}", format_altitude(cruise_altitude));
    println!("Departure Airport ICAO: {}", departure_airport);
    println!("  Arrival Airport ICAO: {}", arrival_airport);
    println!("                 Route: {}", route);
    println!("              Aircraft: {} - {}", aircraft.icao(), aircraft);
    println!("            Passengers: {}", number_passengers);
    println!("                   ZFW: {}", zero_fuel_weight.separate_with_commas());

    LogEntry {
        id,
        planned_departure_time,
        planned_arrival_time,
        flight_number,
        airline,
        cruise_altitude,
        departure_airport,
        arrival_airport,
        distance_nm,
        route,
        aircraft,
        number_passengers,
        zero_fuel_weight: zero_fuel_weight,
    }

}


#[derive(Debug, Clone, PartialEq)]
enum Screen {
    MainMenu,
    BuildLogEntry,
    ViewLogbook,
    ViewStatistics,
    Exit,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Screen::MainMenu => write!(f, "Main Menu"),
            Screen::BuildLogEntry => write!(f, "Create New Flight Plan"),
            Screen::ViewLogbook => write!(f, "View Log Entries"),
            Screen::ViewStatistics => write!(f, "View Statistics"),
            Screen::Exit => write!(f, "Exit"),
        }
    }
}

fn view_logbook(logbook: &Vec<LogEntry>) {
    for entry in logbook {
        println!("    Assigned ID Number: {}", entry.id);
        println!("Planned departure time: {}", entry.planned_departure_time);
        println!("  Planned arrival time: {}", entry.planned_arrival_time);
        println!("         Flight number: {}-{}", entry.airline.icao(), entry.flight_number);
        println!("          Airline: {}", entry.airline);
        println!("       Cruise Altitude: {}", format_altitude(entry.cruise_altitude));
        println!("Departure Airport ICAO: {}", entry.departure_airport);
        println!("  Arrival Airport ICAO: {}", entry.arrival_airport);
        println!("                 Route: {}", entry.route);
        println!("              Aircraft: {} - {}", entry.aircraft.icao(), entry.aircraft);
        println!("            Passengers: {}", entry.number_passengers);
        println!("                   ZFW: {}", entry.zero_fuel_weight.separate_with_commas());   
        println!();
    }
}

fn load_existing_log_entries(file_path: &str) -> Vec<LogEntry> {
    match fs::read_to_string(file_path) {
        Ok(json_contents) => {
            match serde_json::from_str::<Vec<LogEntry>>(&json_contents) {
                Ok(logbook) => {
                    println!("Successfully loaded {} entries from the logbook.", logbook.len());
                    logbook
                }
                Err(e) => {
                    eprintln!("Error parsing the logbook JSON: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            // Check specifically if the error is because the file doesn't exist
            if e.kind() == ErrorKind::NotFound {
                println!("No existing logbook found. A new logbook has been created.");
                Vec::new()
            } else {
                eprintln!("An error occurred when attempting to read the logbook: {}", e);
                Vec::new()
            }
        }
    }
}


fn main_menu() -> Screen {

    let options = vec![
        Screen::BuildLogEntry,
        Screen::ViewLogbook,
        Screen::ViewStatistics,
        Screen::Exit
    ];

    let selection = Select::new("What would you like to do?", options)
        .prompt()
        .expect("Failed to read selection");
    println!();
    return selection;
}

fn save_logbook(logbook: &Vec<LogEntry>) {
    let mut json = serde_json::to_string_pretty(logbook)
        .expect("Error saving logbook to disk.");
    json.push('\n');
    fs::write(FILENAME, json)
        .expect("Error writing logbook to disk");
}


fn calculate_total_miles(logbook: &[LogEntry]) -> u32 {
    logbook.iter().map(|entry| entry.distance_nm).sum()
}

fn get_statistics(logbook: &Vec<LogEntry>) -> String {
    format!("Total Miles Flown: {} NM", calculate_total_miles(logbook))
}

fn main() {

    // Load flights from json if they exist, or else create a new blank logbook.
    let mut logbook = load_existing_log_entries(FILENAME);

    let mut finished = false;
    let mut current_screen = Screen::MainMenu;

    while !finished {
        let next_screen = match current_screen {
            Screen::MainMenu => main_menu(),
            Screen::BuildLogEntry => {
                let new_entry = build_log_entry();
                logbook.push(new_entry);
                save_logbook(&logbook);
                main_menu()
            },
            Screen::ViewLogbook => {
                view_logbook(&logbook);
                main_menu()
            },
            Screen::ViewStatistics => {
                println!();
                println!("{}", &get_statistics(&logbook));
                println!();
                main_menu()
            }
            Screen::Exit => { finished = true; Screen::Exit },
        };

        current_screen = next_screen;
    }

}
