use std::io;
pub enum Degree {
    Fahrenheit(f64),
    Celsius(f64),
}
impl Degree {
    pub fn convert(&self) -> Degree {
        match self {
            Degree::Fahrenheit(num) => Degree::Celsius((num - 32.0) / 1.8),
            Degree::Celsius(num) => Degree::Fahrenheit(num * 1.8 + 32.0),
        }
    }
    pub fn new() -> Degree {
        loop {
            println!("Choose your option:");
            println!("[1] Convert fahrenheit to celsius;");
            println!("[2] Convert celsius to fahrenheit;");
            let mut degree_type = String::new();
            io::stdin().read_line(&mut degree_type)
                .expect("Failed to read the line");
            break match degree_type.trim().parse() {
                Ok(1) => Degree::Fahrenheit(get_degree_value()),
                Ok(2) => Degree::Celsius(get_degree_value()),
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
