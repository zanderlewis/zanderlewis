use rand::Rng;
use std::{fs::OpenOptions, io::Read};
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
        "{{number_of_the_day}}",
        &format!("{}", number_of_the_day),
    );

    // Write the updated content back to README.md
    std::fs::write("README.md", new_content).expect("Unable to write data");
}