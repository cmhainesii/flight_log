use chrono::NaiveDateTime;
use inquire::CustomUserError;
use inquire::Text;
use inquire::validator::Validation;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
struct SimpleObject {
    id: u32,
    name: String,
    is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FlightLog {
    id: Uuid,
    planned_departure_time: String,
    planned_arrival_time: String,
    flight_number: String,
    cruise_altitude: String,
    departure_airport: String,
    arrival_airport: String,
    route: String,
    aircraft_type: String,
    number_passengers: u32,
    zero_fuel_weight: f64,
    max_zfw: f64,
}

fn main() {


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
        .expect("Invalid flight number.");

    println!("Assigned ID Number: {}", id);
    println!("Planned departure time: {}", planned_departure_time);
    println!("Planned arrival time: {}", planned_arrival_time);
    println!("Flight number: {}", flight_number);


    



    // 1. Create rust object
    let original_object = SimpleObject {
        id: 42,
        name: String::from("Example Item"),
        is_active: true,
    };

    // 2. Serialize the object to a JSON string
    let json_string = serde_json::to_string(&original_object)
        .expect("Failed to serialize object to JSON");

    println!("Serialized JSON:\n{}\n", json_string);

    // 3. Deserialize the JSON string back into a Rust object
    let restored_object: SimpleObject = serde_json::from_str(&json_string)
        .expect("Failed to deserialize JSON to object");

    println!("Restored Rust Object:\n{:?}", restored_object);

    


}
