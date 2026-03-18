use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("[=====Degree Conversion Tool=====]");
    loop {
        // Retrieve the user's choice from the selection menu
        let mode: char = choose_mode();
        println!("Chosen: [{}]", mode);

        match mode {
            '1' => {
                println!("Please enter the degree in fahrenheit");
                // enter_value() handles input, convert_to_celsius() handles logic
                println!(
                    "The degree in celsius: {}C\n",
                    convert_to_celsius(enter_value())
                )
            }
            '2' => {
                println!("Please enter the degree in celsius");
                println!(
                    "The degree in fahrenheit: {}F\n",
                    convert_to_fahrenheit(enter_value())
                )
            }
            '3' => {
                println!("Program terminated");
                return; // Exit the loop and the program
            }
            _ => {
                // This case is technically unreachable due to logic in choose_mode()
                println!("Error has occured and program is shutting down");
                panic!("Unhandled input")
            }
        }
        // Pause for 2 seconds before showing the menu again for better UX
        sleep(Duration::from_secs(2));
    }
}

/// Prompts the user for a numerical input and validates that it is a valid f64.
/// Repeatedly loops until a valid number is provided.
fn enter_value() -> f64 {
    loop {
        let mut degree_input: String = String::new();

        io::stdin()
            .read_line(&mut degree_input)
            .expect("Failed to read line");

        // Trim whitespace and attempt to parse as a 64-bit float
        let degree_input: f64 = match degree_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        return degree_input;
    }
}

/// Displays the selection menu and ensures the user selects 1, 2, or 3.
fn choose_mode() -> char {
    println!("Please Choose:\n[1] Convert from fahrenheit to celsius\n[2] Convert from celsius to fahrenheit\n[3] Exit program");
    loop {
        let mut mode_choice: String = String::new();

        io::stdin()
            .read_line(&mut mode_choice)
            .expect("Failed to read line");

        // Parse the input into a char and match against valid options
        let mode_choice: char = match mode_choice.trim().parse() {
            Ok(mode @ ('1' | '2' | '3')) => mode,
            Ok(_) => {
                println!("Please choose correct option");
                continue;
            }
            Err(_) => continue,
        };

        return mode_choice;
    }
}

/// Formula: (F - 32) * 5/9
fn convert_to_celsius(fahrenheit_degree: f64) -> f64 {
    println!("The degree in fahrenheit: {}F", fahrenheit_degree);
    (fahrenheit_degree - 32.0) * 5.0 / 9.0
}

/// Formula: (C * 9/5) + 32
fn convert_to_fahrenheit(celsius_degree: f64) -> f64 {
    println!("The degree in celsius: {}C", celsius_degree);
    (celsius_degree * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;
    // Using the approx crate to handle floating point precision in comparisons
    use approx::assert_relative_eq;

    #[test]
    fn test_celsius_convertor() {
        // epsilon defines the allowed margin of error for floating point results
        assert_relative_eq!(convert_to_celsius(123.0), 50.555, epsilon = 0.01);
        assert_relative_eq!(convert_to_celsius(86.9), 30.5);
        assert_relative_eq!(convert_to_celsius(-46.84), -43.8);
        assert_relative_eq!(convert_to_celsius(-11.83), -24.35);
        assert_relative_eq!(convert_to_celsius(1830.2), 999.0);
    }

    #[test]
    fn test_fahrenheit_convertor() {
        assert_relative_eq!(convert_to_fahrenheit(0.0), 32.0);
        assert_relative_eq!(convert_to_fahrenheit(100.0), 212.0);
        assert_relative_eq!(convert_to_fahrenheit(37.0), 98.6);
    }
}
