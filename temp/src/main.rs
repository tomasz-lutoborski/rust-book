use std::io;

fn main() {
    println!("Enter temperature: ");
    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read temperature");

    let temp: f64 = temp.trim().parse().expect("Not a number");

    println!("Enter unit of input [ c/f ]");
    let mut unit = String::new();

    io::stdin().read_line(&mut unit).expect("Failed to read unit");

    let unit = unit.trim();

    if unit == "c" {
         let temp_out = temp * 1.8 + 32.0;
         println!("Temperature in fahrenheit is equal {}", temp_out);
    }
    else if unit == "f" {
        let temp_out = (temp - 32.0) * 5.0/9.0;
        println!("Temperature in celsius is equal {}", temp_out);
    }
    else {
        println!("Wrong unit");
    }
}
