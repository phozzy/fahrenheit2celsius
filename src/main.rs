use std::io;
enum DegreeType {
    Fahrenheit,
    Celsius,
}
struct DegreeValue {
    kind: DegreeType,
    value: f64,
}
fn get_degree_kind() -> DegreeType {
    loop {
        println!("Choose your option:");
        println!("[1] Convert fahrenheit to celsius;");
        println!("[2] Convert celsius to fahrenheit;");
        let mut degree_type = String::new();
        io::stdin().read_line(&mut degree_type)
            .expect("Failed to read the line");
        break match degree_type.trim().parse() {
            Ok(1) => DegreeType::Fahrenheit,
            Ok(2) => DegreeType::Celsius,
            Ok(_) => {
                println!("Choose one of these values: 1 or 2!");
                continue;
            }
            Err(_) => {
                println!("Wrong value!");
                continue;
            },
        };
    }
}
fn get_degree_value() -> f64 {
    loop {
        println!("Enter the value for your temperature:");
        let mut degree = String::new();
        io::stdin().read_line(&mut degree)
            .expect("Failed to read the line");
        break match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong value!");
                continue;
            },
        };
    }
}
fn fahrenheit2celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
fn celsius2fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}
fn convert_degree(degree: DegreeValue) -> DegreeValue {
    match degree.kind {
        DegreeType::Celsius => DegreeValue {
            kind: DegreeType::Fahrenheit,
            value: celsius2fahrenheit(degree.value),
        },
        DegreeType::Fahrenheit => DegreeValue {
            kind: DegreeType::Celsius,
            value: fahrenheit2celsius(degree.value),
        },
    }
}
fn main() {
    let degree = DegreeValue {
        kind: get_degree_kind(),
        value: get_degree_value(),
    };
    let result_degree = convert_degree(degree);
    match result_degree.kind {
        DegreeType::Fahrenheit => println!("Your degree in Fahrenheit will be:"),
        DegreeType::Celsius => println!("Your degree in Celsius will be:"),
    };
    println!("{}", result_degree.value);
}
