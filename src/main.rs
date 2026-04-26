use core::fmt;
use std::vec;

use chrono::NaiveDateTime;
use inquire::CustomType;
use inquire::CustomUserError;
use inquire::Select;
use inquire::Text;
use inquire::validator::Validation;
use thousands::Separable;
use uuid::Uuid;


mod log_book;
mod log_entry;
mod airline;
mod aircraft;

use log_book::LogBook;
use log_entry::LogEntry;
use airline::Airline;
use aircraft::Aircraft;

const FILENAME: &str = "logbook.nbc";

fn format_altitude(altitude: u32) -> String {
    if altitude >= 18_000 {
        format!("FL{:03}", altitude / 100)
    } else {
        format!("{} ft", altitude.separate_with_commas())
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
        .with_placeholder("2026-04-20 14:30")
        .with_validator(datetime_validator)
        .prompt()
        .expect("Invalid departure time");

    println!("Planned departure time: {}", planned_departure_time);

    let planned_arrival_time = Text::new("Planned arrival time?")
        .with_placeholder("2026-04-20 18:00")
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

    let remarks = Text::new("Remarks?")
        .with_placeholder("Delayed due to ground delay at KBOS.")
        .with_help_message("Enter any additional notes or comments about the flight")
        .prompt()
        .expect("Error parsing pilot remarks.");

    
    

    let new_entry = LogEntry {
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
        zero_fuel_weight,
        remarks
    };


    println!("    Assigned ID Number: {}", new_entry.id);
    println!("                Flight: {} -> {}", new_entry.departure_airport, new_entry.arrival_airport);
    println!("Planned departure time: {}", new_entry.planned_departure_time);
    println!("  Planned arrival time: {}", new_entry.planned_arrival_time);
    println!("         Flight number: {}-{}", new_entry.airline.icao(), new_entry.flight_number);
    println!("               Airline: {}", new_entry.airline);
    println!("       Cruise Altitude: {}", format_altitude(new_entry.cruise_altitude));
    println!("Departure Airport ICAO: {}", new_entry.departure_airport);
    println!("  Arrival Airport ICAO: {}", new_entry.arrival_airport);
    println!("                 Route: {}", new_entry.route);
    println!("              Aircraft: {} - {}", new_entry.aircraft.icao(), new_entry.aircraft);
    println!("            Passengers: {}", new_entry.number_passengers);
    println!("                   ZFW: {}", new_entry.zero_fuel_weight.separate_with_commas());
    println!("                  Load: {:.2}%", new_entry.get_load_percent());
    println!("    Passenger Capacity: {:.2}%", new_entry.get_psx_percent());
    println!("         Pilot Remarks: {}", new_entry.remarks);
    println!();

    return new_entry;

    

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






fn main() {

    // Load flights from json if they exist, or else create a new blank logbook.
    let mut logbook = LogBook::load_existing_log_entries(FILENAME);

    let mut finished = false;
    let mut current_screen = Screen::MainMenu;

    while !finished {
        let next_screen = match current_screen {
            Screen::MainMenu => main_menu(),
            Screen::BuildLogEntry => {
                let new_entry = build_log_entry();
                logbook.add(new_entry);
                logbook.save();
                main_menu()
            },
            Screen::ViewLogbook => {
                logbook.print_logbook();
                main_menu()
            },
            Screen::ViewStatistics => {
                println!();
                println!("{}", logbook.get_statistics());
                println!();
                main_menu()
            }
            Screen::Exit => { finished = true; Screen::Exit },
        };

        current_screen = next_screen;
    }

}
