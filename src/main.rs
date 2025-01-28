use rand::Rng;
use std::{fs::OpenOptions, io::Read};

fn main() {
    // Seed the random number generator with the current date
    let mut rng = rand::rng();
    let number_of_the_hour: i32 = rng.random_range(-999999..=999999);

    // Read the existing content of README.md
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("README.md")
        .expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read data");

    // Replace the placeholder with the new number of the hour
    let new_content = content.replace(
        "{{number_of_the_hour}}",
        &format!("{}", number_of_the_hour),
    );

    // Write the updated content back to README.md
    std::fs::write("README.md", new_content).expect("Unable to write data");
}