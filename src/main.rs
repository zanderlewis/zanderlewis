use rand::Rng;
use std::{fs::File, io::Write};
use chrono::prelude::*;

fn main() {
    // Get the current date
    let local: DateTime<Local> = Local::now();
    let date_str = local.format("%Y-%m-%d").to_string();

    // Seed the random number generator with the current date
    let mut rng = rand::thread_rng();
    let number_of_the_day: i32 = rng.gen_range(-10000..=10000);

    // Write the number to nod.txt (delete the file if it already exists)
    let mut file = File::create("nod.txt").expect("Unable to create file");
    let content = format!("Number of the day for {}: {}", date_str, number_of_the_day);
    file.write_all(content.as_bytes()).expect("Unable to write data");
}
