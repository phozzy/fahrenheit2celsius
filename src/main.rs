use fahrenheit_and_celsius::degrees::Degree;
fn main() {
    let degree = Degree::new();
    match degree.convert() {
        Degree::Fahrenheit(num) => println!("Your degree in Fahrenheit will be: {}", num),
        Degree::Celsius(num) => println!("Your degree in Celsius will be: {}", num),
    };
}
