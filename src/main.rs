use sha256gen::run;
use std::env;
fn main() {
    let args = env::args();
    if let Err(err) = run(args) {
        eprintln!("Program ran into an error: {err}");
        std::process::exit(1);
    }
}
