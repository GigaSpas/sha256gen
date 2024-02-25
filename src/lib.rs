use std::{fs::File, io::Write};

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    args.next();
    let file_path;
    match args.next() {
        Some(a) => file_path = a,
        None => return Err("No file path provided"),
    }
    let content = std::fs::read_to_string(file_path).unwrap();
    let mut output = String::new();
    content
        .lines()
        .for_each(|a| output += &(a.to_owned() + " " + &sha256::digest(a) + "\n"));
    let mut file_new = File::create("output.txt").unwrap();
    file_new.write_all(output.as_bytes()).unwrap();
    Ok(())
}
