use rand::Rng;
use std::{fs::OpenOptions, io::{Read, Write}};
#[allow(unused_imports)]
use chrono::prelude::*;

fn main() {
    // Get the current date
    let local: DateTime<Local> = Local::now();
    #[allow(unused_variables)]
    let date_str = local.format("%Y-%m-%d").to_string();

    // Seed the random number generator with the current date
    let mut rng = rand::thread_rng();
    let number_of_the_day: i32 = rng.gen_range(-10000..=10000);

    // Read the existing content of README.md
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("README.md")
        .expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read data");

    // Replace the placeholder with the new number of the day
    let new_content = content.replace(
        "### Number of the day: {{number_of_the_day}}",
        &format!("### Number of the day: {}", number_of_the_day),
    );

    // Write the updated content back to README.md
    file.set_len(0).expect("Unable to truncate file");
    file.write_all(new_content.as_bytes()).expect("Unable to write data");
}