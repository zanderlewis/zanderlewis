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

    // Write content to README.md in place of `### Number of the day: {{number_of_the_day}}`
    let content = format!("### Number of the day: {}", number_of_the_day);
    let mut file = File::create("README.md").expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}
