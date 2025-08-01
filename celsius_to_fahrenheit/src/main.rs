fn main() {
  let celsius = 100.0;
  println!("{} degrees Celsius is {} degrees Fahrenheit", celsius, celsius_to_fahrenheit(celsius))
}

fn celsius_to_fahrenheit(celsius: f64) -> f64{
  celsius * 9.0 / 5.0 + 32.0
}