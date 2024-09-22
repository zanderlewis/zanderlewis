use rand::Rng;
use chrono::prelude::*;

fn main() {
    // Get the current date
    let local: DateTime<Local> = Local::now();
    let date_str = local.format("%Y-%m-%d").to_string();

    // Seed the random number generator with the current date
    let mut rng = rand::thread_rng();
    let number_of_the_day: i32 = rng.gen_range(-10000..=10000);

    println!("Number of the day for {}: {}", date_str, number_of_the_day);
}
