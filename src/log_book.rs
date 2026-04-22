use std::{fs, io::ErrorKind};
use std::fmt::Write;

use thousands::Separable;

use crate::LogEntry;
pub struct LogBook {
    logs: Vec<LogEntry>

}

impl LogBook {
    pub fn add(&mut self, entry: LogEntry) {
        self.logs.push(entry);
    }

    pub fn save(&self) {
        let mut json = serde_json::to_string_pretty(&self.logs)
            .expect("Error saving logbook to disk.");
        json.push('\n');
        fs::write(crate::FILENAME, json)
            .expect("Error writing logbook to disk");
    }


    pub fn calculate_total_miles(&self) -> u32 {
        self.logs.iter().map(|entry| entry.distance_nm).sum()
    }

    pub fn load_existing_log_entries(file_path: &str) -> LogBook {
        match fs::read_to_string(file_path) {
            Ok(json_contents) => {
                match serde_json::from_str::<Vec<LogEntry>>(&json_contents) {
                    Ok(logbook) => {
                        println!("Successfully loaded {} entries from the logbook.", logbook.len());
                        LogBook { logs: logbook }
                    }
                    Err(e) => {
                        eprintln!("Error parsing the logbook JSON: {}", e);
                        LogBook { logs: Vec::new() }
                    }
                }
            }
            Err(e) => {
                // Check specifically if the error is because the file doesn't exist
                if e.kind() == ErrorKind::NotFound {
                    println!("No existing logbook found. A new logbook has been created.");
                    LogBook { logs: Vec::new() }
                } else {
                    eprintln!("An error occurred when attempting to read the logbook: {}", e);
                    LogBook { logs: Vec::new() }
                }
            }
        }
    }

    pub fn get_statistics(&self) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "        Total Miles Flown: {} NM", self.calculate_total_miles());
        let _ = writeln!(output, "     Average Load Percent: {:.02} %", self.calculate_average_load_percent());
        let _ = writeln!(output, "Average Passenger Percent: {:.02} %", self.calculate_average_psx_percent());
        output
    }

    pub fn calculate_average_load_percent(&self) -> f64 {
        let mut sum = 0;
        for entry in &self.logs {
            sum += entry.get_load_percent() as i32;
        }
        return sum as f64 / self.logs.len() as f64
    }

    pub fn calculate_average_psx_percent(&self) -> f64 {
        let mut sum = 0;
        for entry in &self.logs {
            sum += entry.get_psx_percent() as i32;
        }
        return sum as f64 / self.logs.len() as f64;
    }

    pub fn print_logbook(&self) {
        for (index, entry) in self.logs.iter().enumerate() {
            println!("      Log Entry Number: {}", index + 1);
            println!("    Assigned ID Number: {}", entry.id);
            println!("                Flight: {} --> {}", entry.departure_airport, entry.arrival_airport);
            println!("Planned departure time: {}", entry.planned_departure_time);
            println!("  Planned arrival time: {}", entry.planned_arrival_time);
            println!("         Flight number: {}-{}", entry.airline.icao(), entry.flight_number);
            println!("          Airline: {}", entry.airline);
            println!("       Cruise Altitude: {}", crate::format_altitude(entry.cruise_altitude));
            println!("Departure Airport ICAO: {}", entry.departure_airport);
            println!("  Arrival Airport ICAO: {}", entry.arrival_airport);
            println!("                 Route: {}", entry.route);
            println!("              Aircraft: {} - {}", entry.aircraft.icao(), entry.aircraft);
            println!("            Passengers: {}", entry.number_passengers);
            println!("                   ZFW: {}", entry.zero_fuel_weight.separate_with_commas());
            println!("                  Load: {:.2}%", entry.get_load_percent());
            println!("    Passenger Capacity: {:.2}%", entry.get_psx_percent());
            println!("         Pilot Remarks: {}", entry.remarks);
            println!();
        }
    }
}